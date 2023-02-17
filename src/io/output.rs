use crate::errors;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, IOType, OutputType};
use crate::io::{
    DeviceMetadata, IODirection, IOEvent, IOKind, IdTraits, IdType,
};
use crate::storage::Container;
use crate::storage::{Containerized, MappedCollection, OwnedLog};
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

/// Interface defining an output device
/// Implementing output devices can be done through structs and can be stored in a container via
/// `Containerized` trait. Any structs that implement this trait may be accessed by `OutputType`
///
/// # Functions
/// - `tx() -> IOEvent`: Low-level function for passing object to device.
/// - `write() -> Result<()>`: Main interface function output device. Should update cached state.
/// - `state() -> IOType`: Get cached state of output device. Facade for `state` field that gets updated by `write()`.
///
/// # Notes:
/// Since `Containerized` is implemented for the `Output` trait, types that implement the `Output` trait
/// can be stored in a container returned by the `Containerized::container()` method. This way, multiple instances of
/// differing types may be stored in the same `Container`.
///
/// ```
/// let mut container = Containerized::<Box<dyn crate::Output<f32>>, i32>::container();
/// container.insert(1, Box::new(HeatingPad::new(String::from("Temperature Output"), 1)));
/// container.insert(2, Box::new(Humidifier::new(String::from("Humidity Output"), 2)));
/// ```
/// > Note how two different output device types were stored in `container`.
pub trait Output: Device {
    fn tx(&self, value: &IOType) -> IOEvent;
    fn write(&mut self, event: &IOType) -> errors::Result<IOEvent>;
    fn state(&self) -> &IOType;
}

impl<K> Containerized<Deferred<OutputType>, K> for OutputType
where
    K: IdTraits,
{
    fn container() -> Container<Deferred<OutputType>, K> {
        Container::<Deferred<OutputType>, K>::new()
    }
}

impl std::fmt::Debug for OutputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Output {{ name: {}, id: {}, kind: {}",
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}

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
    type Inner = OutputType;
    /// Return wrapped `OutputType` in `Deferred`
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(Box::new(self)))
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

impl Output for GenericOutput {
    /// Return a mock value
    fn tx(&self, value: &IOType) -> IOEvent {
        /** low-level functionality goes here **/
        self.generate_event(Utc::now(), Some(*value))
    }

    /// Primary interface method during polling.
    /// Calls `tx()`, updates cached state, and saves to log.
    fn write(&mut self, value: &IOType) -> errors::Result<IOEvent> {
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
    fn state(&self) -> &IOType {
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