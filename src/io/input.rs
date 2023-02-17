use crate::errors;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::{IdTraits, InputType, IOType};
use crate::io::{Device, DeviceMetadata, IdType, IODirection, IOEvent, IOKind};
use crate::storage::{Container, Containerized, MappedCollection, OwnedLog};
use chrono::{DateTime, Utc};
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
use crate::action::{Publisher, PublisherInstance};

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

    /// Generate an `IOEvent` instance from provided value or `::rx()`
    fn generate_event(&self, dt: DateTime<Utc>, value: Option<IOType>) -> IOEvent {
        IOEvent::generate(self, dt, value.unwrap_or_else(move || self.rx()))
    }

    fn read(&mut self, time: DateTime<Utc>) -> errors::Result<IOEvent>;

    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) -> Result<(), ()>;
    fn has_publisher(&self) -> bool;
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
        IOType::Float(1.2)
    }

    /// Get IOEvent, add to log, and propagate to publisher/subscribers
    /// Primary interface method during polling.
    fn read(&mut self, time: DateTime<Utc>) -> errors::Result<IOEvent> {
        // get IOEvent
        let event = self.generate_event(time, None);

        // propagate to publisher/subscribers
        match &self.publisher {
            Some(publisher) => publisher.lock().unwrap().notify(&event),
            _ => ()
        };

        // add to log
        let mut binding = self.log.lock().unwrap();
        binding.push(time, event)?;
        Ok(event)
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
    fn has_publisher(&self) -> bool {
        match self.publisher {
            Some(_) => true,
            None => false
        }
    }
}


// Testing
#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use crate::action::PublisherInstance;
    use crate::helpers::Deferrable;
    use crate::io::{Device, GenericInput};
    use crate::io::{Input, IOType};

    const DUMMY_OUTPUT: IOType = IOType::Float(1.2);

    #[test]
    fn test_rx() {
        let input = GenericInput::default();
        assert_eq!(input.rx(), DUMMY_OUTPUT);
    }

    #[test]
    fn test_read() {
        let mut input = GenericInput::default();

        let time = Utc::now();
        let event = input.read(time).unwrap();
        assert_eq!(event.data.value, DUMMY_OUTPUT);
        assert_eq!(event.timestamp, time);
        assert_eq!(event.data.kind, input.kind());

        // TODO: attach log and assert that IOEvent has been added to log
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
