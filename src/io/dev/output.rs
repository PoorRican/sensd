use std::fmt::Formatter;
use chrono::{Duration, Utc};
use crate::action::{Command, IOCommand, Routine};
use crate::errors::{ErrorType, no_internal_closure};
use crate::helpers::Def;
use crate::io::{Device, DeviceMetadata, IODirection, IOEvent, IOKind, IdType, RawValue, DeviceGetters, DeviceSetters};
use crate::storage::{Chronicle, Log};

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
/// let name = "our new output device";
/// let kind = IOKind::default();
///
/// let device = Output::new(name, id, kind);
///
/// assert_eq!(device.name(), name);
/// assert_eq!(device.id(), id);
///
/// assert_ne!(device, Output::default());
/// ```
///
/// Now that we are able to set device metadata, constructor methods still don't
/// provide any way to interact with hardware. The builder method [`Device::set_command()`]
/// is used to add low-level code. In this example, we return a static value:
///
/// ```
/// use sensd::action::IOCommand;
/// use sensd::io::{Device, Output, RawValue};
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
    state: Option<RawValue>,
    log: Option<Def<Log>>,
    command: Option<IOCommand>,
}

impl DeviceGetters for Output {
    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    /// Immutable reference to cached state
    ///
    /// `state` field should be updated by `write()`
    fn state(&self) -> &Option<RawValue> {
        &self.state
    }
}

impl DeviceSetters for Output {
    fn set_name<N>(&mut self, name: N) where N: Into<String> {
        self.metadata.name = name.into();
    }

    fn set_id(&mut self, id: IdType) {
        self.metadata.id = id;
    }

    fn set_log(&mut self, log: Def<Log>) {
        self.log = Some(log);
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
    fn new<N, K>(name: N, id: IdType, kind: K) -> Self
    where
        Self: Sized,
        N: Into<String>,
        K: Into<Option<IOKind>>,
    {
        let kind = kind.into().unwrap_or_default();
        let state = None;
        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Out);

        let command = None;
        let log = None;

        Self {
            metadata,
            state,
            log,
            command,
        }
    }

    fn set_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        command.agrees(IODirection::Out)
            .expect("Command is not output");
        self.command = Some(command);
        self
    }
}

impl Output {
    /// Execute low-level GPIO command to write data
    ///
    /// # Parameters
    ///
    /// - `value`: [`RawValue`] to send to device
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
    fn tx(&self, value: RawValue) -> Result<IOEvent, ErrorType> {
        if let Some(command) = &self.command {
            command.execute(Some(value))?;
        } else {
            return Err(no_internal_closure());
        };

        Ok(self.generate_event(value))
    }

    /// Get [`IOEvent`], add to log and update cache.
    ///
    /// Primary interface method called during polling,
    /// and by [`crate::action::Action::evaluate()`] and [`Routine::execute()`].
    ///
    /// # Parameters
    ///
    /// - `value`: [`RawValue`] to write to device. There is no check on value.
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
    /// use sensd::io::{Device, DeviceGetters, Output, RawValue};
    ///
    /// let value = RawValue::default();
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
    pub fn write(&mut self, value: RawValue) -> Result<IOEvent, ErrorType> {
        let event = self.tx(value).expect("Low level device error while writing");

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
    pub fn create_routine(&self, value: RawValue, duration: Duration) -> Routine {
        let timestamp = Utc::now() + duration;
        let log = self.log.as_ref()
            .expect("Output device does not have log")
            .to_owned()
            .clone();
        let command = self.command.as_ref()
            .expect("Output device does not have command")
            .to_owned()
            .clone();
        Routine::new(
            timestamp,
            value,
            log,
            command,
        )
    }
}

impl Chronicle for Output {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use crate::action::IOCommand;
    use crate::io::{Device, DeviceGetters, IOKind, Output, RawValue};
    use crate::storage::Chronicle;

    /// Dummy output command for testing.
    /// Accepts value and returns `Ok(())`
    const COMMAND: IOCommand = IOCommand::Output(move |_| Ok(()));

    #[test]
    /// Test that constructor accepts `name` as `&str` or `String`
    fn new_name_parameter() {
        Output::new("as &str", 0, None);
        Output::new(String::from("as String"), 0, None);
    }

    #[test]
    fn new_kind_parameter() {
        Output::new("", 0, None);
        Output::new("", 0, Some(IOKind::Unassigned));
        Output::new("", 0, IOKind::Unassigned);
    }

    #[test]
    fn test_tx() {
        let mut output = Output::default();
        output.command = Some(COMMAND);

        let value = RawValue::Binary(true);
        let event = output.tx(value).expect("Unknown error occurred in `tx()`");

        assert_eq!(value, event.value);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let mut output = Output::default().init_log();
        let log = output.log().unwrap();

        assert_eq!(log.try_lock().unwrap().iter().count(), 0);

        let value = RawValue::Binary(true);
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
    fn set_root() {
        let output = Output::default().init_log();

        assert!(output.log()
            .unwrap().try_lock().unwrap()
            .root_path()
            .is_none());

        output.set_root(Arc::new(PathBuf::new()));

        assert!(output.log()
            .unwrap().try_lock().unwrap()
            .root_path()
            .is_some());
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