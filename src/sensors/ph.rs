use chrono::Duration;

use crate::{device, io, units::Ph};

#[derive(Debug, Clone)]
pub struct MockPhSensor {
    metadata: device::DeviceMetadata,
}

/** Represents a mock pH sensor.
*/
impl MockPhSensor {
    /// Creates a mock ph sensor which returns random values
    ///
    /// # Arguments
    ///
    /// * `name`: arbitrary name of sensor
    /// * `sensor_id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    pub fn new(name: String, sensor_id: i32) -> Self {
        let version_id = 0;
        let kind = io::IOKind::PH;
        let min_value = 0.0;
        let max_value = 14.0;
        let resolution = 0.1;

        let metadata: device::DeviceMetadata = device::DeviceMetadata::new(
            name, version_id, sensor_id, kind, min_value, max_value, resolution,
        );

        MockPhSensor { metadata }
    }
}

// Implement traits
impl device::Device for MockPhSensor {
    fn get_metadata(&self) -> &device::DeviceMetadata {
        &self.metadata
    }
    fn name(&self) -> String {
        self.metadata.name.clone()
    }
    fn id(&self) -> i32 {
        self.metadata.sensor_id
    }
}

impl device::Sensor for MockPhSensor {
    /// Return a mock value
    fn read(&self) -> f64 {
        1.2
    }
}

impl From<device::DeviceMetadata> for MockPhSensor {
    fn from(metadata: device::DeviceMetadata) -> Self {
        Self { metadata }
    }
}