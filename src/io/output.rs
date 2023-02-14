use crate::errors;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, OutputType};
use crate::io::{
    DeviceMetadata, IODirection, IOEvent, IOKind, IOType, IdTraits, IdType, SubscriberStrategy,
};
use crate::storage::Container;
use crate::storage::{Containerized, MappedCollection, OwnedLog};
use chrono::{DateTime, Utc};
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

/// Interface defining an output device
/// Implementing output devices can be done through structs and can be stored in a container via
/// `Containerized` trait. Any structs that implement this trait may be accessed by `OutputType`
///
/// # Functions
/// - `tx() -> IOType`: Low-level function for interacting with device.
/// - `write() -> Result<()>`: Main interface function output device.
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
    fn write(&mut self, event: &IOEvent) -> errors::Result<()>;
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

        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);

        GenericOutput { metadata, log }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
}

impl Output for GenericOutput {
    /// Return a mock value
    fn tx(&self, event: &IOEvent) -> IOEvent {
        event.clone().invert(1.0)
    }

    /// Primary interface method during polling.
    /// Calls `tx()` and saves to log.
    fn write(&mut self, event: &IOEvent) -> errors::Result<()> {
        let event = self.tx(event);
        // add to log
        self.log
            .lock()
            .unwrap()
            .push(event.timestamp, event.clone())
    }
}
