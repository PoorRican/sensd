use crate::action::{Command, GPIOCommand};
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{
    no_internal_closure, Device, DeviceMetadata, DeviceType, IODirection, IOEvent, IOKind, IOType,
    IdType,
};
use crate::storage::{HasLog, Log};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GenericOutput {
    metadata: DeviceMetadata,
    // cached state
    state: IOType,
    log: Option<Deferred<Log>>,
    command: Option<GPIOCommand>,
}

impl Deferrable for GenericOutput {
    type Inner = DeviceType;
    /// Return wrapped `OutputType` in `Deferred`
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(DeviceType::Output(self)))
    }
}

// Implement traits
impl Device for GenericOutput {
    /// Creates a generic output device
    ///
    /// # Arguments
    ///
    /// * `name`: user given name of device
    /// * `id`: arbitrary, numeric ID to differentiate from other devices
    ///
    /// returns: GenericOutput
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Option<Deferred<Log>>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();
        let state = IOType::default();
        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Output);

        let command = None;

        Self {
            metadata,
            state,
            log,
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

impl GenericOutput {
    /// Execute GPIO command
    fn tx(&self, value: IOType) -> Result<IOEvent, ErrorType> {
        if let Some(command) = &self.command {
            command.execute(Some(value)).unwrap();
        } else {
            return Err(no_internal_closure());
        };

        Ok(self.generate_event(value))
    }

    /// Primary interface method during polling.
    ///
    /// Calls `tx()`, updates cached state, and saves to log.
    ///
    /// # Notes
    /// This method will fail if there is no associated log
    pub fn write(&mut self, value: IOType) -> Result<IOEvent, ErrorType> {
        let event = self.tx(value).expect("Error returned by `tx()`");

        // update cached state
        self.state = event.data.value;

        self.add_to_log(event);

        Ok(event)
    }

    /// Immutable reference to cached state
    /// `state` field should be updated by `write()`
    pub fn state(&self) -> &IOType {
        &self.state
    }
}

impl HasLog for GenericOutput {
    fn log(&self) -> Option<Deferred<Log>> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::action::{GPIOCommand, IOCommand};
    use crate::io::{Device, GenericOutput, IOType};
    use crate::storage::MappedCollection;

    /// Dummy output command for testing.
    /// Accepts value and returns `Ok(())`
    const COMMAND: IOCommand = IOCommand::Output(move |_| Ok(()));

    #[test]
    fn test_tx() {
        let mut output = GenericOutput::default();
        output.command = Some(GPIOCommand::new(COMMAND, None));

        let value = IOType::Binary(true);
        let event = output.tx(value).expect("Unknown error occurred in `tx()`");

        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let mut output = GenericOutput::default();
        let log = output.init_log(None);

        assert_eq!(log.try_lock().unwrap().length(), 0);

        let value = IOType::Binary(true);
        output.command = Some(GPIOCommand::new(COMMAND, None));

        // check `state` before `::write()`
        assert_ne!(value, *output.state());

        let event = output
            .write(value)
            .expect("Unknown error returned by `::write()`");

        // check state after `::write()`
        assert_eq!(value, *output.state());

        // check returned `IOEvent`
        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);

        // assert that event was added to log
        assert_eq!(log.try_lock().unwrap().length(), 1);
    }
}
