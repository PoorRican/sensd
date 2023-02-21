use crate::errors;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, IOType, DeviceType};
use crate::io::{
    DeviceMetadata, IODirection, IOEvent, IOKind, IdTraits, IdType,
};
use crate::storage::Container;
use crate::storage::{MappedCollection, OwnedLog};
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};


pub struct GenericOutput {
    metadata: DeviceMetadata,
    // cached state
    state: IOType,
    pub log: Deferred<OwnedLog>,
}
impl Default for GenericOutput {
    /// Overwrite default value for `IODirection` in `DeviceMetadata`
    fn default() -> Self {
        let mut metadata = DeviceMetadata::default();
        metadata.direction = IODirection::Output;

        let state = IOType::default();
        let log = Arc::new(Mutex::new(OwnedLog::default()));
        Self { metadata, state, log }
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

        GenericOutput { metadata, state, log }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }

    /// Generate an `IOEvent` instance from provided value or `::tx()`
    fn generate_event(&self, dt: DateTime<Utc>, value: Option<IOType>) -> IOEvent {
        IOEvent::generate(self, dt, value.unwrap())
    }
}

impl GenericOutput {
    /// Return a mock value
    pub fn tx(&self, value: &IOType) -> IOEvent {
        /** low-level functionality goes here **/
        self.generate_event(Utc::now(), Some(*value))
    }

    /// Primary interface method during polling.
    /// Calls `tx()`, updates cached state, and saves to log.
    pub fn write(&mut self, value: &IOType) -> errors::Result<IOEvent> {
        let event = self.tx(value);

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
    use chrono::{DateTime, Utc};
    use crate::io::{GenericOutput, IdType, IOData, IODirection, IOEvent, IOType, Output};

    const DUMMY_VALUE: IOType = IOType::Float(1.2);

    fn generate_event(id: IdType, timestamp: DateTime<Utc>, value: IOType) -> IOEvent {
        let direction = IODirection::Input;
        let mut data = IOData::default();
        data.value = value;
        IOEvent { id, timestamp, direction, data }
    }

    #[test]
    fn test_tx() {
        let id: IdType = 3;
        let timestamp = Utc::now();
        let value = IOType::Binary(true);
        let event = generate_event(id, timestamp, value);

        let output = GenericOutput::default();

        let new = output.tx(&value);

        assert_eq!(new.data.value, value);
        assert_eq!(new.data.kind, event.data.kind);
        assert_ne!(new.direction, event.direction);
    }
}