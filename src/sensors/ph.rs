use chrono::Duration;

use crate::{device, io, units::Ph};

pub struct MockPhSensor  {
    info: device::DeviceInfo<Ph>
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
        let min_value = 0.0;
        let max_value = 14.0;
        let resolution = 0.1;

        let info: device::SensorInfo<Ph> = device::SensorInfo::new(name, version_id, sensor_id,
                                                                    kind, min_value, max_value, resolution,
                                                                     min_delay);

        MockPhSensor {
            info
        }
    }
}


// Implement traits
impl device::Device<Ph> for MockPhSensor {}

impl device::Readable<Ph> for MockPhSensor {
    /// Return a mock value
    fn read(&self) -> Ph {
        Ph::new(1.2)
    }
}