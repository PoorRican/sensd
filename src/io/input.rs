use chrono::{DateTime, Utc};
use crate::{errors, io};
use crate::io::Device;
use crate::storage::{Container, Containerized};

/// Interface for an input device
/// It is used as a trait object and can be stored in a container using the `Containerized` trait.
///
/// # Type Parameters
/// - `T`: The type of data that the sensor will produce.
///
/// # Functions
/// - `read() -> T`: Reads the sensor and return the data as a value of type `T`.
/// - `get_event() -> &IOEvent`: Create an `IOEvent` with current sensor data.
///
/// # Examples
/// ```
/// let sensor: &dyn crate::Sensor<f32> = TemperatureSensor::new(String::from("Temperature Sensor"), 1);
/// let reading = sensor.read();
/// let info = sensor.get_info();
/// ```
///
/// # Notes:
/// Since `Containerized` is implemented for the `Sensor` trait, any types that implement the `Sensor` trait
/// can be stored in a container using the `Containerized::container()` method. This way, multiple instances of
/// differing types may be stored in the same `Container`.
///
/// ```
/// let mut container = Containerized::<Box<dyn crate::Sensor<f32>>, i32>::container();
/// container.insert(1, Box::new(TemperatureSensor::new(String::from("Temperature Sensor"), 1)));
/// container.insert(2, Box::new(HumiditySensor::new(String::from("Humidity Sensor"), 2)));
/// ```
/// > Note how two different sensor types were stored in `container`.
pub trait Input: Device {
    fn read(&self) -> f64;

    fn get_event(&self, dt: DateTime<Utc>) -> io::IOEvent {
        io::IOEvent::create(self, dt, self.read())
    }

    fn poll(&mut self, time: DateTime<Utc>) -> errors::Result<()>;
}

/// Returns a new instance of `Container` for objects with `Sensor` trait indexed by `K`.
/// Sensor traits are stored as `Box<dyn Sensor>`
impl<K> Containerized<Box<dyn Input>, K> for dyn Input
where
    K: std::hash::Hash + Eq,
{
    fn container() -> Container<Box<dyn Input>, K> {
        Container::<Box<dyn Input>, K>::new()
    }
}
