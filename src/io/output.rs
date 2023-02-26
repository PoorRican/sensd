use crate::action::GPIOCommand;
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{DeviceMetadata, IODirection, IOEvent, IOKind, IdType, Device, IOType, DeviceType, no_internal_closure};
use crate::storage::{MappedCollection, OwnedLog};
use std::sync::{Arc, Mutex};


pub struct GenericOutput {
    metadata: DeviceMetadata,
    // cached state
    state: IOType,
    pub log: Deferred<OwnedLog>,
    command: Option<GPIOCommand>,
}
impl Default for GenericOutput {
    /// Overwrite default value for `IODirection` in `DeviceMetadata`
    fn default() -> Self {
        let mut metadata = DeviceMetadata::default();
        metadata.direction = IODirection::Output;

        let state = IOType::default();
        let log = Arc::new(Mutex::new(OwnedLog::default()));
        let command = None;
        Self { metadata, state, log, command }
    }
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
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();
        let state = IOType::default();
        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);

        let command = None;

        Self { metadata, state, log, command }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    fn add_command(&mut self, command: GPIOCommand) {
        self.command = Some(command);
    }
}

impl GenericOutput {
    /// Return a mock value
    pub fn tx(&self, value: IOType) -> Result<IOEvent, ErrorType> {
        // Execute GPIO command
        if let Some(command) = &self.command {
            command.execute(Some(value)).unwrap();
        } else { return Err(no_internal_closure()) };

        Ok(self.generate_event(value))
    }

    /// Primary interface method during polling.
    /// Calls `tx()`, updates cached state, and saves to log.
    pub fn write(&mut self, value: IOType) -> Result<IOEvent, ErrorType> {
        let event = self.tx(value).expect("Error returned by `tx()`");

        // update cached state
        self.state = event.data.value;

        // add to log
        self.log
            .lock()
            .unwrap()
            .push(event.timestamp, event.clone())?;
        Ok(event)
    }

    /// Immutable reference to cached state
    /// `state` field should be updated by `write()`
    pub fn state(&self) -> &IOType {
        &self.state
    }
}


#[cfg(test)]
mod tests {
    use crate::action::{GPIOCommand, IOCommand};
    use crate::io::{Device, GenericOutput, IOType};

    /// Dummy output command for testing.
    /// Accepts value and returns `Ok(())`
    const COMMAND: IOCommand = IOCommand::Output(move |val| Ok(()));

    #[test]
    fn test_tx() {
        let value = IOType::Binary(true);
        let mut output = GenericOutput::default();
        output.command = Some(GPIOCommand::new(COMMAND, None));

        let event = output.tx(value).expect("Unknown error occurred in `tx()`");

        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let value = IOType::Binary(true);
        let mut output = GenericOutput::default();
        output.command = Some(GPIOCommand::new(COMMAND, None));

        // check `state` before `::write()`
        assert_ne!(value, *output.state());

        let new = output.write(value).expect("Unknown error returned by `::write()`");

        // check state after `::write()`
        assert_eq!(value, *output.state());

        // check returned `IOEvent`
        assert_eq!(value, new.data.value);
        assert_eq!(output.kind(), new.data.kind);
        assert_eq!(output.direction(), new.direction);

        // TODO: attach log and assert that event was added to log
    }
}