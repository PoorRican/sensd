use crate::action::{Action, Command, IOCommand, Publisher};
use crate::errors::DeviceError;
use crate::helpers::Def;
use crate::io::dev::device::set_log_dir;
use crate::io::{
    Datum, Device, DeviceGetters, DeviceMetadata, DeviceSetters, IODirection, IOEvent, IOKind,
    IdType,
};
use crate::name::Name;
use crate::storage::{Chronicle, Directory, Log};
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

#[derive(Default)]
/// This is the generic implementation for any external input device.
///
/// # Getting Started
///
/// While [`Input`] derives a [`Default`] implementation, `name` and `id`
/// should be passed to [`Device::new()`] constructor to differentiate it
/// from other [`Input`] objects.
///
/// ```
/// use sensd::io::{Device, DeviceGetters, Input, IOKind};
/// let id = 777;
///
/// let input = Input::new(id);
///
/// assert_eq!(input.id(), id);
/// assert_ne!(input, Input::default());
/// ```
///
/// Now that we are able to set device metadata, constructor methods still don't
/// provide any way to interact with hardware. The builder method [`Device::set_command()`]
/// is used to add low-level code. In this example, we return a static value:
///
/// ```
/// use sensd::action::IOCommand;
/// use sensd::io::{Device, Input, Datum};
///
/// let command = IOCommand::Input(|| Datum::binary(true));
/// let input =
///     Input::default()
///         .set_command(command);
/// ```
///
/// With a `command` set, [`Input::read()`] can be used to generate [`IOEvent`] objects
/// from input data.
pub struct Input {
    metadata: DeviceMetadata,
    log: Option<Def<Log>>,
    publisher: Option<Publisher>,
    command: Option<IOCommand>,
    state: Option<Datum>,

    dir: Option<PathBuf>,
}

/// Implement unique constructors and builder methods
impl Device for Input {
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    ///
    /// - `name`: arbitrary name of sensor
    /// - `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// # Returns
    ///
    /// Partially initialized [`Input`]. The builder method [`Device::set_command()`]
    /// needs to be called to assign an [`IOCommand`] to interact with hardware.
    fn new(id: IdType) -> Self {
        let metadata: DeviceMetadata = DeviceMetadata::new(id, IODirection::In);

        let publisher = None;
        let command = None;
        let log = None;
        let state = None;

        let dir = None;

        Self {
            metadata,
            log,
            publisher,
            command,
            state,
            dir,
        }
    }

    fn set_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        command
            .agrees(IODirection::In)
            .expect("Command is not input");
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

impl Name for Input {
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

impl Directory for Input {
    fn parent_dir(&self) -> Option<PathBuf> {
        self.dir.clone()
    }

    /// Setter for parent dir
    ///
    /// Updates any internal field that needs a parent directory (ie: [`Log`])
    ///
    /// # Parameters
    ///
    /// - `path`: New path to store as parent dir
    ///
    /// # Returns
    ///
    /// Ownership of `Self` with `parent_dir` set to allow method chaining.
    fn set_parent_dir_ref<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        self.dir = PathBuf::from(path).into();

        set_log_dir(self.log(), self.full_path());

        self
    }
}

impl DeviceGetters for Input {
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

impl DeviceSetters for Input {
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

impl Input {
    /// Execute low-level GPIO command to read data
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
    fn rx(&self) -> Result<IOEvent, DeviceError> {
        let read_value = if let Some(command) = &self.command {
            // execute command
            let result = command.execute(None)?;
            // return error if no value is read from device
            match result {
                None => Err(DeviceError::ValueExpected {
                    metadata: self.metadata.clone(),
                })?,
                Some(inner) => inner,
            }
        } else {
            Err(DeviceError::NoCommand {
                metadata: self.metadata.clone(),
            })?
        };

        Ok(IOEvent::new(read_value))
    }

    /// Propagate `IOEvent` to all subscribers.
    ///
    /// Silently fails when there is no associated publisher.
    ///
    /// # Parameters
    ///
    /// - `event`: A reference to [`IOEvent`] to propagate to subscribed [`Action`]'s
    fn propagate(&mut self, event: &IOEvent) {
        if let Some(publisher) = &mut self.publisher {
            publisher.propagate(&event);
        };
    }

    /// Get IOEvent, add to log, and propagate to publisher/subscribers
    ///
    /// Primary interface method during polling.
    ///
    /// # Notes
    ///
    /// A panic is not thrown if there is no log associated.
    ///
    /// # Panics
    ///
    /// - If there is an error when reading from sensor on a low-level
    ///
    /// # Returns
    ///
    /// A [`Result`] containing:
    ///
    /// - `Ok` with [`IOEvent`] if read was successful
    /// - `Err` with [`ErrorType`] if read failed
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::action::IOCommand;
    /// use sensd::io::{Device, DeviceGetters, Input, Datum};
    ///
    /// let value = Datum::default();
    /// let command = IOCommand::Input(|| Datum::default());
    /// let mut input = Input::default().set_command(command);
    ///
    /// let event = input.read().unwrap();
    ///
    /// assert_eq!(event.value, value);
    ///
    /// // cached state is updated
    /// assert_eq!(input.state().unwrap(), value);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Publisher::propagate()`] for how [`IOEvent`] is given to subscribing [`Action`]'s
    /// - [`Input::push_to_log()`] for adding [`IOEvent`] to [`Log`]
    pub fn read(&mut self) -> Result<IOEvent, DeviceError> {
        let event = self.rx()?;

        // Update cached state
        self.state = Some(event.value);

        self.propagate(&event);
        self.push_to_log(&event);

        Ok(event)
    }

    /// Create and set publisher or silently fail
    pub fn init_publisher(mut self) -> Self {
        match self.publisher {
            None => {
                self.publisher = Some(Publisher::default());
            }
            _ => {
                eprintln!("Publisher already exists!");
            }
        }
        self
    }

    /// Get mutable reference to internal [`Publisher`] instance
    ///
    /// # See Also
    ///
    /// [`Input::subscribe()`] for subscribing actions to an [`Input`]
    pub fn publisher_mut(&mut self) -> Option<&mut Publisher> {
        self.publisher.as_mut()
    }

    pub fn publisher(&self) -> Option<&Publisher> {
        self.publisher.as_ref()
    }

    pub fn has_publisher(&self) -> bool {
        match self.publisher {
            Some(_) => true,
            None => false,
        }
    }

    /// Helper function for subscribing controllers to internal [`Publisher`] instance
    ///
    /// # Parameters
    ///
    /// `action`: Any implementation of [`Action`]
    ///
    /// # Returns
    ///
    /// Returns ownership to `self`
    ///
    /// # Example
    ///
    /// For adding a simple threshold
    /// ```
    /// use sensd::action::actions::Threshold;
    /// use sensd::action::{Action, Trigger};
    /// use sensd::io::{Datum, Device, Input, Output};
    ///
    /// let output = Output::default().into_deferred();
    /// let input =
    ///     Input::default()
    ///         .init_publisher()
    ///         .subscribe(Threshold::new(
    ///             "test threshold",
    ///             Datum::int(80),
    ///             Trigger::GT
    ///         ).set_output(output));
    /// assert!(input.has_publisher());
    /// assert_eq!(input.publisher().unwrap().subscribers().len(), 1)
    /// ```
    pub fn subscribe<A: Action>(mut self, action: A) -> Self {
        if let Some(publisher) = self.publisher_mut() {
            publisher.subscribe(action.into_boxed())
        }
        self
    }
}

impl Chronicle for Input {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input Device - {{ name: {}, id: {}, kind: {}}}",
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata && self.command == other.command
    }
}

// Testing
#[cfg(test)]
mod tests {
    use crate::action::IOCommand;
    use crate::io::{Datum, Device, Input};
    use crate::storage::{Chronicle, Directory, Document};

    const DUMMY_OUTPUT: Datum = Datum::Float(Some(1.2));
    const COMMAND: IOCommand = IOCommand::Input(move || DUMMY_OUTPUT);

    #[test]
    fn test_rx() {
        let mut input = Input::default();

        input.command = Some(COMMAND);

        let event = input.rx().unwrap();
        assert_eq!(event.value, DUMMY_OUTPUT);
    }

    #[test]
    fn test_read() {
        let mut input = Input::default().init_log();
        let log = input.log();

        input.command = Some(COMMAND);

        assert_eq!(log.clone().unwrap().try_lock().unwrap().iter().count(), 0);

        let event = input.read().unwrap();
        assert_eq!(event.value, DUMMY_OUTPUT);

        // assert that event was added to log
        assert_eq!(log.unwrap().try_lock().unwrap().iter().count(), 1);
    }

    /// Test `::add_publisher()` and `::has_publisher()`
    #[test]
    fn test_init_publisher() {
        let mut input = Input::default();

        assert_eq!(false, input.has_publisher());

        input = input.init_publisher();

        assert_eq!(true, input.has_publisher());
    }

    #[test]
    fn test_init_log() {
        let mut input = Input::default();

        assert_eq!(false, input.has_log());

        input = input.init_log();

        assert_eq!(true, input.has_log());
    }

    #[test]
    /// Test that [`Input::set_parent_dir()`] correctly changes [`Log::dir()`]
    fn set_dir_changes_log_dir() {
        let mut input = Input::default().init_log();

        assert!(input.log().unwrap().try_lock().unwrap().dir().is_none());

        input = input.set_parent_dir("");

        assert!(input.log().unwrap().try_lock().unwrap().dir().is_some());
    }
}
