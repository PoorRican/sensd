use crate::action::{Command, GPIOCommand, Publisher, PublisherInstance};
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::DeviceType;
use crate::io::{
    no_internal_closure, Device, DeviceMetadata, IODirection, IOEvent, IOKind, IdType,
};
use crate::storage::{HasLog, Log};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GenericInput {
    metadata: DeviceMetadata,
    log: Option<Deferred<Log>>,
    publisher: Option<Deferred<PublisherInstance>>,
    command: Option<GPIOCommand>,
}

impl Deferrable for GenericInput {
    type Inner = DeviceType;
    /// Return wrapped Sensor in
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(DeviceType::Input(self)))
    }
}

// Implement traits
impl Device for GenericInput {
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    /// * `name`: arbitrary name of sensor
    /// * `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Option<Deferred<Log>>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);
        let publisher = None;
        let command = None;

        Self {
            metadata,
            log,
            publisher,
            command,
        }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    fn add_command(&mut self, command: GPIOCommand) {
        self.command = Some(command);
    }

    fn add_log(&mut self, log: Deferred<Log>) {
        self.log = Some(log)
    }
}

impl GenericInput {
    /// Return a mock value
    pub fn rx(&self) -> Result<IOEvent, ErrorType> {
        // Execute GPIO command
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
        if let Some(publisher) = &self.publisher {
            publisher.lock().unwrap().notify(&event);
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

        self.add_to_log(event);

        Ok(event)
    }

    pub fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) -> Result<(), ()> {
        match self.publisher {
            None => {
                self.publisher = Some(publisher);
                Ok(())
            }
            _ => Err(()),
        }
    }
    pub fn has_publisher(&self) -> bool {
        match self.publisher {
            Some(_) => true,
            None => false,
        }
    }
}

impl HasLog for GenericInput {
    fn log(&self) -> Option<Deferred<Log>> {
        self.log.clone()
    }
}

// Testing
#[cfg(test)]
mod tests {
    use crate::action::{GPIOCommand, IOCommand, PublisherInstance};
    use crate::helpers::Deferrable;
    use crate::io::{Device, GenericInput, IOType};
    use crate::storage::MappedCollection;

    const DUMMY_OUTPUT: IOType = IOType::Float(1.2);
    const COMMAND: IOCommand = IOCommand::Input(move || DUMMY_OUTPUT);

    #[test]
    fn test_rx() {
        let mut input = GenericInput::default();

        input.command = Some(GPIOCommand::new(COMMAND, None));

        let event = input.rx().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
    }

    #[test]
    fn test_read() {
        let mut input = GenericInput::default();
        let log = input.init_log(None);

        input.command = Some(GPIOCommand::new(COMMAND, None));

        assert_eq!(log.try_lock().unwrap().length(), 0);

        let event = input.read().unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
        assert_eq!(event.data.kind, input.kind());

        // assert that event was added to log
        assert_eq!(log.try_lock().unwrap().length(), 1);
    }

    /// Test `::add_publisher()` and `::has_publisher()`
    #[test]
    fn test_add_publisher() {
        let mut input = GenericInput::default();

        assert_eq!(false, input.has_publisher());

        let publisher = PublisherInstance::default();
        input.add_publisher(publisher.deferred()).unwrap();

        assert_eq!(true, input.has_publisher());
    }
}
