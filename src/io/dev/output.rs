use crate::action::{Command, IOCommand, Routine};
use crate::errors::{DeviceError, ErrorType};
use crate::helpers::Def;
use crate::io::dev::device::set_log_dir;
use crate::io::{
    Datum, Device, DeviceGetters, DeviceMetadata, DeviceSetters, IODirection, IOEvent, IOKind,
    IdType,
};
use crate::name::Name;
use crate::storage::{Chronicle, Directory, Log};
use chrono::{Duration, Utc};
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

#[derive(Default)]
/// This is the generic implementation for any external output device.
///
/// # Getting Started
///
/// While [`Output`] derives a [`Default`] implementation, `name` and `id`
/// should be passed to [`Device::new()`] constructor to differentiate it
/// from other [`Output`] objects.
///
/// ```
/// use sensd::io::{Device, DeviceGetters, Output, IOKind};
/// let id = 777;
///
/// let device = Output::new(id);
///
/// assert_eq!(device.id(), id);
/// assert_ne!(device, Output::default());
/// ```
///
/// Now that we are able to set device metadata, constructor methods still don't
/// provide any way to interact with hardware. The builder method [`Device::set_command()`]
/// is used to add low-level code. In this example, we return a static value:
///
/// ```
/// use sensd::action::IOCommand;
/// use sensd::io::{Device, Output, Datum};
///
/// let command = IOCommand::Output(|_| Ok(()));
/// let device =
///     Output::default()
///         .set_command(command);
/// ```
///
/// With a `command` set, [`Output::write()`] can be used to actuate or send data
/// to devices.
pub struct Output {
    metadata: DeviceMetadata,
    // cached state
    state: Option<Datum>,
    log: Option<Def<Log>>,
    command: Option<IOCommand>,

    dir: Option<PathBuf>,
}

impl Name for Output {
    fn name(&self) -> &String {
        &self.metadata().name
    }

    fn set_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.metadata.name = name.into();
        self
    }
}

impl Directory for Output {
    fn parent_dir(&self) -> Option<PathBuf> {
        self.dir.clone()
    }

    /// Setter for parent directory
    ///
    /// Updates any internal field that needs a parent directory (ie: [`Log`])
    ///
    /// # Parameters
    ///
    /// - `path`: New [`PathBuf`] to store
    fn set_parent_dir_ref<P>(&mut self, path: P) -> &mut Self
    where
        Self: Sized,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        self.dir = Some(PathBuf::from(path.clone()));

        set_log_dir(self.log(), self.full_path());

        self
    }
}

impl DeviceGetters for Output {
    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    /// Immutable reference to cached state
    ///
    /// `state` field should be updated by `write()`
    fn state(&self) -> &Option<Datum> {
        &self.state
    }
}

impl DeviceSetters for Output {
    fn set_id(&mut self, id: IdType) {
        self.metadata.id = id;
    }

    fn set_log(&mut self, log: Def<Log>) {
        self.log = Some(log.clone());

        if let Some(dir) = &self.dir {
            set_log_dir(Some(log), dir)
        }
    }
}

/// Implement unique constructors and builder methods
impl Device for Output {
    /// Creates a generic output device
    ///
    /// # Arguments
    ///
    /// * `name`: user given name of device
    /// * `id`: arbitrary, numeric ID to differentiate from other devices
    ///
    /// returns: GenericOutput
    fn new(id: IdType) -> Self {
        let state = None;
        let metadata: DeviceMetadata = DeviceMetadata::new(id, IODirection::Out);

        let command = None;
        let log = None;
        let dir = None;

        Self {
            metadata,
            state,
            log,
            command,
            dir,
        }
    }

    fn set_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        command
            .agrees(IODirection::Out)
            .expect("Command is not output");
        self.command = Some(command);
        self
    }

    fn set_kind(mut self, kind: IOKind) -> Self
    where
        Self: Sized,
    {
        self.metadata.kind = kind;
        self
    }
}

impl Output {
    /// Execute low-level GPIO command to write data
    ///
    /// # Parameters
    ///
    /// - `value`: [`Datum`] to send to device
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    ///
    /// - `Ok` with [`IOEvent`] if low-level operation occurred successfully.
    /// - `Err` with [`ErrorType`] if no `command` is set or there device command failed
    ///
    /// # Issues
    ///
    /// [Low level error type](https://github.com/PoorRican/sensd/issues/192)
    fn tx(&self, value: Datum) -> Result<IOEvent, DeviceError> {
        if let Some(command) = &self.command {
            command.execute(Some(value))?;
        } else {
            Err(DeviceError::NoCommand {
                metadata: self.metadata.clone(),
            })?;
        };

        Ok(IOEvent::new(value))
    }

    /// Get [`IOEvent`], add to log and update cache.
    ///
    /// Primary interface method called during polling,
    /// and by [`crate::action::Action::evaluate()`] and [`Routine::execute()`].
    ///
    /// # Parameters
    ///
    /// - `value`: [`Datum`] to write to device. There is no check on value.
    ///
    /// # Notes
    ///
    /// A panic is not thrown if there is no log associated.
    ///
    /// # Panics
    ///
    /// - If there is an error when writing to device on a low-level
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::action::IOCommand;
    /// use sensd::io::{Device, DeviceGetters, Output, Datum};
    ///
    /// let value = Datum::default();
    /// let command = IOCommand::Output(|_| Ok(()));
    /// let mut output = Output::default().set_command(command);
    ///
    /// let event = output.write(value).unwrap();
    ///
    /// assert_eq!(event.value, value);
    ///
    /// // cached state is updated
    /// assert_eq!(output.state().unwrap(), value);
    /// ```
    ///
    /// # Issues
    ///
    /// [Low level error type](https://github.com/PoorRican/sensd/issues/192)
    ///
    /// # See Also
    ///
    /// - [`Input::push_to_log()`] for adding [`IOEvent`] to [`Log`]
    pub fn write(&mut self, value: Datum) -> Result<IOEvent, ErrorType> {
        let event = self
            .tx(value)
            .expect("Low level device error while writing");

        // update cached state
        self.state = Some(event.value);

        self.push_to_log(&event);

        Ok(event)
    }

    /// Create a [`Routine`] given a value to write and a duration
    ///
    /// # Parameters
    ///
    /// - `value`: Value to write to device
    /// - `duration`: Duration to wait before executing action.
    ///
    /// # Returns
    ///
    /// [`Routine`] ready to be added to [`crate::action::SchedRoutineHandler`]
    pub fn create_routine(&self, value: Datum, duration: Duration) -> Routine {
        let timestamp = Utc::now() + duration;
        let log = self
            .log
            .as_ref()
            .expect("Output device does not have log")
            .to_owned()
            .clone();
        let command = self
            .command
            .as_ref()
            .expect("Output device does not have command")
            .to_owned()
            .clone();
        Routine::new(timestamp, value, log, command)
    }
}

impl Chronicle for Output {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Output Device: - {{ name: {}, id: {}, kind: {}}}",
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata && self.command == other.command
    }
}

#[cfg(test)]
mod tests {
    use crate::action::IOCommand;
    use crate::io::{Datum, Device, DeviceGetters, Output};
    use crate::storage::{Chronicle, Directory, Document};

    /// Dummy output command for testing.
    /// Accepts value and returns `Ok(())`
    const COMMAND: IOCommand = IOCommand::Output(move |_| Ok(()));

    #[test]
    fn test_tx() {
        let mut output = Output::default();
        output.command = Some(COMMAND);

        let value = Datum::binary(true);
        let event = output.tx(value).expect("Unknown error occurred in `tx()`");

        assert_eq!(value, event.value);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let mut output = Output::default().init_log();
        let log = output.log().unwrap();

        assert_eq!(log.try_lock().unwrap().iter().count(), 0);

        let value = Datum::binary(true);
        output.command = Some(COMMAND);

        // check `state` before `::write()`
        assert_eq!(None, *output.state());

        let event = output
            .write(value)
            .expect("Unknown error returned by `::write()`");

        // check state after `::write()`
        assert_eq!(value, output.state().unwrap());

        // check returned `IOEvent`
        assert_eq!(value, event.value);

        // assert that event was added to log
        assert_eq!(log.try_lock().unwrap().iter().count(), 1);
    }

    #[test]
    fn test_init_log() {
        let mut output = Output::default();

        assert_eq!(false, output.has_log());

        output = output.init_log();

        assert_eq!(true, output.has_log());
    }

    #[test]
    fn set_dir() {
        let mut output = Output::default().init_log();

        assert!(output.log().unwrap().try_lock().unwrap().dir().is_none());

        output = output.set_parent_dir("");

        assert!(output.log().unwrap().try_lock().unwrap().dir().is_some());
    }

    #[test]
    /// Test that [`Input::set_parent_dir()`] correctly changes [`Log::dir()`]
    fn set_dir_changes_log_dir() {
        let mut output = Output::default().init_log();

        assert!(output.log().unwrap().try_lock().unwrap().dir().is_none());

        output = output.set_parent_dir("");

        assert!(output.log().unwrap().try_lock().unwrap().dir().is_some());
    }
}
