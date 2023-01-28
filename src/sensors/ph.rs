use chrono::Duration;

use crate::{device, io, units::Ph};

#[derive(Debug, Clone)]
pub struct MockPhSensor {
    metadata: device::DeviceMetadata<Ph>,
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
    /// * `min_delay`: minimum delay between sensor reads
    ///
    /// returns: MockPhSensor
    pub fn new(name: String, sensor_id: i32, min_delay: Duration) -> Self {
        let version_id = 0;
        let kind = io::IOKind::PH;
        let min_value = Ph(0.0);
        let max_value = Ph(14.0);
        let resolution = Ph(0.1);

        let metadata: device::DeviceMetadata<Ph> = device::DeviceMetadata::new(
            name, version_id, sensor_id, kind, min_value, max_value, resolution, min_delay,
        );

        MockPhSensor { metadata }
    }
}

// Implement traits
impl device::Device<Ph> for MockPhSensor {
    fn get_metadata(&self) -> &device::DeviceMetadata<Ph> {
        &self.metadata
    }
    fn name(&self) -> String {
        self.metadata.name.clone()
    }
    fn id(&self) -> i32 {
        self.metadata.sensor_id
    }
}

impl device::Sensor<Ph> for MockPhSensor {
    /// Return a mock value
    fn read(&self) -> Ph {
        Ph::new(1.2).unwrap()
    }
}
