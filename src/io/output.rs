use std::fmt::Formatter;
use crate::action::{Command, IOCommand};
use crate::errors::{ErrorType, no_internal_closure};
use crate::helpers::Def;
use crate::io::{Device, DeviceMetadata, IODirection, IOEvent, IOKind, IdType, RawValue};
use crate::storage::{Chronicle, Log};

#[derive(Default)]
pub struct Output {
    metadata: DeviceMetadata,
    // cached state
    state: Option<RawValue>,
    log: Option<Def<Log>>,
    command: Option<IOCommand>,
}

// Implement traits
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

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    fn set_name<N>(&mut self, name: N) where N: Into<String> {
        self.metadata.name = name.into();
    }

    fn set_id(&mut self, id: IdType) {
        self.metadata.id = id;
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

    fn set_log(&mut self, log: Def<Log>) {
        self.log = Some(log)
    }

    /// Immutable reference to cached state
    ///
    /// `state` field should be updated by `write()`
    fn state(&self) -> &Option<RawValue> {
        &self.state
    }
}

impl Output {
    /// Execute low-level GPIO command
    fn tx(&self, value: RawValue) -> Result<IOEvent, ErrorType> {
        if let Some(command) = &self.command {
            command.execute(Some(value))?;
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
        self.state = Some(event.data.value);

        self.push_to_log(event);

        Ok(event)
    }
}

impl Chronicle for Output {
    fn log(&self) -> Option<Def<Log>> {
        self.log.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::action::IOCommand;
    use crate::io::{Device, IOKind, Output, RawValue};
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

        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);
    }

    #[test]
    /// Test that `tx()` was called, cached state was updated, and IOEvent added to log.
    fn test_write() {
        let mut output = Output::default().init_log(None);
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
        assert_eq!(value, event.data.value);
        assert_eq!(output.kind(), event.data.kind);
        assert_eq!(output.direction(), event.direction);

        // assert that event was added to log
        assert_eq!(log.try_lock().unwrap().iter().count(), 1);
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
