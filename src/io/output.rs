use crate::action::{Command, IOCommand};
use crate::errors::ErrorType;
use crate::helpers::Def;
use crate::io::{
    no_internal_closure, Device, DeviceMetadata, DeviceType, IODirection, IOEvent, IOKind, IdType,
    RawValue,
};
use crate::storage::{Chronicle, Log};

#[derive(Default)]
pub struct GenericOutput {
    metadata: DeviceMetadata,
    // cached state
    state: RawValue,
    log: Option<Def<Log>>,
    command: Option<IOCommand>,
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
    fn new(name: String, id: IdType, kind: Option<IOKind>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();
        let state = RawValue::default();
        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Output);

        let command = None;
        let log = None;

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

    fn add_command(mut self, command: IOCommand) -> Self
    where
        Self: Sized,
    {
        self.command = Some(command);
        self
    }

    fn set_log(&mut self, log: Def<Log>) {
        self.log = Some(log)
    }

    fn into_variant(self) -> DeviceType {
        DeviceType::Output(self)
    }
}

impl GenericOutput {
    /// Execute low-level GPIO command
    fn tx(&self, value: RawValue) -> Result<IOEvent, ErrorType> {
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
    pub fn write(&mut self, value: RawValue) -> Result<IOEvent, ErrorType> {
        let event = self.tx(value).expect("Error returned by `tx()`");

        // update cached state
        self.state = event.data.value;

        self.add_to_log(event);

        Ok(event)
    }

    /// Immutable reference to cached state
    /// `state` field should be updated by `write()`
    pub fn state(&self) -> &RawValue {
        &self.state
    }
}

impl Chronicle for GenericOutput {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::action::IOCommand;
    use crate::io::{Device, GenericOutput, RawValue};
    use crate::storage::Chronicle;

    /// Dummy output command for testing.
    /// Accepts value and returns `Ok(())`
    const COMMAND: IOCommand = IOCommand::Output(move |_| Ok(()));

    #[test]
    fn test_tx() {
        let mut output = GenericOutput::default();
        output.command = Some(COMMAND);

        let value = RawValue::Binary(true);
        let event = output.tx(value).expect("Unknown error occurred in `tx()`");

        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let mut output = GenericOutput::default().init_log(None);
        let log = output.log().unwrap();

        assert_eq!(log.try_lock().unwrap().iter().count(), 0);

        let value = RawValue::Binary(true);
        output.command = Some(COMMAND);

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
        assert_eq!(log.try_lock().unwrap().iter().count(), 1);
    }
}
