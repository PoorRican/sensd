use crate::errors;
use crate::helpers::Deferred;
use crate::io::{event, Device, IdTraits, InputType, IOType};
use crate::storage::{Container, Containerized};
use chrono::{DateTime, Utc};
use std::fmt::Formatter;

/// Interface for an input device
/// It is used as a trait object and can be stored in a container using the `Containerized` trait.
/// Any structs that implement this trait may be accessed by `InputType`
///
/// # Functions
/// - `rx() -> IOType`: Low-level function for interacting with device.
/// - `read() -> Result<()>`: Main interface function called during polling.
/// - `generate_event() -> &IOEvent`: Create an `IOEvent` with data from `rx()`.
///
/// # Notes:
/// Since `Containerized` is implemented for the `Input` trait, therefore types that implement the `Input` trait
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

    fn generate_event(&self, dt: DateTime<Utc>, value: Option<IOType>) -> event::IOEvent {
        event::IOEvent::create(self, dt, value.unwrap_or_else(move || self.rx()))
    }

    fn read(&mut self, time: DateTime<Utc>) -> errors::Result<()>;
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
