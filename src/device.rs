/// Provide Low-level Device Functionality
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fmt::Formatter;

use crate::container::{Container, Containerized};
use crate::errors::Result;
use crate::io;

/// Basic interface for GPIO device metadata
pub trait Device {
    fn get_metadata(&self) -> &DeviceMetadata;
    fn name(&self) -> String;
    fn id(&self) -> i32;
}

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
pub trait Sensor: Device {
    fn read(&self) -> f64;

    fn get_event(&self, dt: DateTime<Utc>) -> io::IOEvent {
        io::IOEvent::create(self, dt, self.read())
    }

    fn poll(&mut self, time: DateTime<Utc>) -> Result<()>;
}

impl std::fmt::Debug for dyn Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor {{ name: {}, id: {}, kind: {}",
            self.name(),
            self.id(),
            self.get_metadata().kind
        )
    }
}

/// Defines an interface for an input device that needs to be calibrated
pub trait Calibrated {
    /// Initiate the calibration procedures for a specific device instance.
    fn calibrate(&self) -> bool;
}

/// Encapsulation of individual device metadata
///
/// This struct stores information about a device, including its name, version ID, sensor ID,
/// kind, minimum and maximum value, and resolution.
///
/// # Example
///
/// ```
/// let name = "PH Sensor".to_string();
/// let version_id = 0;
/// let sensor_id = 1;
/// let kind = crate::io::IOKind::PH;
/// let min_value = 0.0;
/// let max_value = 14.0;
/// let resolution = 0.1;
///
/// let info = crate::DeviceInfo::new(name, version_id, sensor_id, kind, min_value, max_value, resolution);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMetadata {
    // TODO: what changes should be made? Dedicated struct for number space?
    pub name: String,
    pub version_id: i32,
    pub sensor_id: i32,
    pub kind: io::IOKind,
}

impl DeviceMetadata {
    /// Creates a new instance of `DeviceInfo`
    ///
    /// # Arguments
    ///
    /// * `name`: name of device
    /// * `version_id`: driver or interface version ID
    /// * `sensor_id`: sensor ID of the device (should be arbitrary)
    /// * `kind`: IOKind representing device type
    /// * `min_value`: measurable or theoretical minimum value (in SI units)
    /// * `max_value`: measurable or theoretical maximum value (in SI units)
    /// * `resolution`: measurable resolution of the device
    ///
    /// # Returns
    ///
    /// A new instance with given specified parameters
    pub fn new(
        name: String,
        version_id: i32,
        sensor_id: i32,
        kind: io::IOKind,
    ) -> Self {
        DeviceMetadata {
            name,
            version_id,
            sensor_id,
            kind,
        }
    }
}

impl std::fmt::Display for DeviceMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device Info {{ Kind: {} }}",
            self.kind,
        )
    }
}

/// Returns a new instance of `Container` for storing objects which implement the `Sensor` trait which are accessed ``
/// Objects are stored as `Box<dyn Sensor>`
impl<K> Containerized<Box<dyn Sensor>, K> for dyn Sensor
where
    K: std::hash::Hash + Eq,
{
    fn container() -> Container<Box<dyn Sensor>, K> {
        Container::<Box<dyn Sensor>, K>::new()
    }
}
