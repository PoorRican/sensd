use std::fmt::Formatter;
use crate::action::{Command, IOCommand, Publisher};
use crate::errors::{ErrorType, no_internal_closure};
use crate::helpers::Def;
use crate::io::{Device, DeviceMetadata, IODirection, IOEvent, IOKind, IdType, RawValue, DeviceGetters, DeviceSetters};
use crate::storage::{Chronicle, Log};

#[derive(Default)]
pub struct Input {
    metadata: DeviceMetadata,
    log: Option<Def<Log>>,
    publisher: Option<Publisher>,
    command: Option<IOCommand>,
    state: Option<RawValue>,
}

// Implement traits
impl Device for Input {
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    /// * `name`: arbitrary name of sensor
    /// * `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new<N, K>(name: N, id: IdType, kind: K) -> Self
    where
        Self: Sized,
        N: Into<String>,
        K: Into<Option<IOKind>>,
    {
        let kind = kind.into().unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name.into(), id, kind, IODirection::In);

        let publisher = None;
        let command = None;
        let log = None;
        let state = None;

        Self {
            metadata,
            log,
            publisher,
            command,
            state,
        }
    }

    fn set_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        command.agrees(IODirection::In)
            .expect("Command is not input");
        self.command = Some(command);
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
    fn state(&self) -> &Option<RawValue> {
        &self.state
    }
}

impl DeviceSetters for Input {
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

impl Input {
    /// Execute low-level GPIO command
    fn rx(&self) -> Result<IOEvent, ErrorType> {
        let read_value = if let Some(command) = &self.command {
            let result = command.execute(None).unwrap();
            result.unwrap()
        } else {
            return Err(no_internal_closure());
        };

        Ok(self.generate_event(read_value))
    }

    /// Propagate `IOEvent` to all subscribers.
    ///
    /// No error is raised when there is no associated publisher.
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
    /// This method will fail if there is no associated log
    pub fn read(&mut self) -> Result<IOEvent, ErrorType> {
        let event = self.rx().expect("Error returned by `rx()`");

        self.propagate(&event);

        self.push_to_log(event);

        Ok(event)
    }

    /// Create and set publisher or silently fail
    pub fn init_publisher(mut self) -> Self
    where
        Self: Sized {
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

    pub fn publisher_mut(&mut self) -> &mut Option<Publisher> {
        &mut self.publisher
    }

    pub fn publisher(&self) -> &Option<Publisher> {
        &self.publisher
    }

    pub fn has_publisher(&self) -> bool {
        match self.publisher {
            Some(_) => true,
            None => false,
        }
    }
}

impl Chronicle for Input {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

// Testing
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::action::{IOCommand};
    use crate::io::{Device, DeviceGetters, Input, IOKind, RawValue};
    use crate::settings::Settings;
    use crate::storage::Chronicle;

    const DUMMY_OUTPUT: RawValue = RawValue::Float(1.2);
    const COMMAND: IOCommand = IOCommand::Input(move || DUMMY_OUTPUT);

    #[test]
    /// Test that constructor accepts `name` as `&str` or `String`
    fn new_name_parameter() {
        Input::new("as &str", 0, None);
        Input::new(String::from("as String"), 0, None);
    }

    #[test]
    fn new_kind_parameter() {
        Input::new("", 0, None);
        Input::new("", 0, Some(IOKind::Unassigned));
        Input::new("", 0, IOKind::Unassigned);
    }

    #[test]
    fn test_rx() {
        let mut input = Input::default();

        input.command = Some(COMMAND);

        let event = input.rx().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
    }

    #[test]
    fn test_read() {
        let mut input = Input::default().init_log(None);
        let log = input.log();

        input.command = Some(COMMAND);

        assert_eq!(log.clone().unwrap().try_lock().unwrap().iter().count(), 0);

        let event = input.read().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
        assert_eq!(event.data.kind, input.kind());

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
        // test w/ None
        {
            let mut input = Input::default();

            assert_eq!(false, input.has_log());

            input = input.init_log(None);

            assert_eq!(true, input.has_log());
        }

        // test `Into<_>` conversion
        {
            let mut input = Input::default();

            assert_eq!(false, input.has_log());

            input = input.init_log(Arc::new(Settings::default()));

            assert_eq!(true, input.has_log());
        }

        // test wrapping in `Some(_)`
        {
            let mut input = Input::default();

            assert_eq!(false, input.has_log());

            input = input.init_log(Some(Arc::new(Settings::default())));

            assert_eq!(true, input.has_log());
        }
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