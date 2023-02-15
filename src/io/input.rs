use crate::errors;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::{IOType, IdTraits, InputType};
use crate::io::{Device, DeviceMetadata, IODirection, IOEvent, IOKind, IdType, Publisher, PublisherInstance};
use crate::storage::{Container, Containerized, MappedCollection, OwnedLog};
use chrono::{DateTime, Utc};
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

/// Interface defining an input device
/// It is used as a trait object and can be stored in a container using the `Containerized` trait.
/// Any structs that implement this trait may be accessed by `InputType`
///
/// # Functions
/// - `rx() -> IOType`: Low-level function for interacting with device.
/// - `read() -> Result<()>`: Main interface function called during polling.
/// - `generate_event() -> &IOEvent`: Create an `IOEvent` with data from `rx()`.
///
/// # Notes:
/// Since `Containerized` is implemented for the `Input` trait, types that implement the `Input` trait
/// can be stored in a container using the `Containerized::container()` method. This way, multiple instances of
/// differing types may be stored in the same `Container`.
///
/// ```
/// let mut container = Containerized::<Box<dyn crate::Input<f32>>, i32>::container();
/// container.insert(1, Box::new(TemperatureSensor::new(String::from("Temperature Sensor"), 1)));
/// container.insert(2, Box::new(HumiditySensor::new(String::from("Humidity Sensor"), 2)));
/// ```
/// > Note how two different sensor types were stored in `container`.
pub trait Input: Device {
    fn rx(&self) -> IOType;

    fn generate_event(&self, dt: DateTime<Utc>, value: Option<IOType>) -> IOEvent {
        IOEvent::generate(self, dt, value.unwrap_or_else(move || self.rx()))
    }

    fn read(&mut self, time: DateTime<Utc>) -> errors::Result<()>;

    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) -> Result<(), ()>;
}

/// Returns a new instance of `Container` for `InputType` indexed by `K`.
/// Input traits are stored as `Deferred<InputType>`
impl<K> Containerized<Deferred<InputType>, K> for InputType
where
    K: IdTraits,
{
    fn container() -> Container<Deferred<InputType>, K> {
        Container::<Deferred<InputType>, K>::new()
    }
}

impl std::fmt::Debug for dyn Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input {{ name: {}, id: {}, kind: {}",
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}

#[derive(Default)]
pub struct GenericInput {
    metadata: DeviceMetadata,
    pub log: Deferred<OwnedLog>,
    publisher: Option<Deferred<PublisherInstance>>,
}

impl Deferrable for GenericInput {
    type Inner = InputType;
    /// Return wrapped Sensor in
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(Box::new(self)))
    }
}

// Implement traits
impl Device for GenericInput {
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    ///
    /// * `name`: arbitrary name of sensor
    /// * `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self
    where
        Self: Sized,
    {
        let kind = kind.unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);
        let publisher = None;

        GenericInput {
            metadata,
            log,
            publisher,
        }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
}

impl Input for GenericInput {
    /// Return a mock value
    fn rx(&self) -> IOType {
        1.2
    }

    /// Get IOEvent, add to log, and propagate to publisher/subscribers
    /// Primary interface method during polling.
    fn read(&mut self, time: DateTime<Utc>) -> errors::Result<()> {
        // get IOEvent
        let event = self.generate_event(time, None);

        // propagate to publisher/subscribers
        match &self.publisher {
            Some(publisher) => publisher.lock().unwrap().notify(&event),
            _ => ()
        }

        // add to log
        let result = self.log.lock().unwrap().push(time, event);

        result
    }

    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) -> Result<(), ()> {
        match self.publisher {
            None => {
                self.publisher = Some(publisher);
                Ok(())
            },
            _ => Err(())
        }
    }
}
