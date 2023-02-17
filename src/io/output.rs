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
    fn tx(&self, event: &IOEvent) -> IOEvent;
    fn write(&mut self, event: &IOEvent) -> errors::Result<IOEvent>;
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

#[derive(Default)]
pub struct GenericOutput {
    metadata: DeviceMetadata,
    // cached state
    state: IOType,
    pub log: Deferred<OwnedLog>,
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
}

impl Output for GenericOutput {
    /// Return a mock value
    fn tx(&self, event: &IOEvent) -> IOEvent {
        let val = IOType::Float(1.2);
        event.clone().invert(val)
    }

    /// Primary interface method during polling.
    /// Calls `tx()`, updates state, and saves to log.
    fn write(&mut self, event: &IOEvent) -> errors::Result<IOEvent> {
        let event = self.tx(event);

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
