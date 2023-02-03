use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{device, io};
use crate::container::{Collection, Container, Containerized};
use crate::io::IOEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPhSensor {
    metadata: device::DeviceMetadata,
    log: Container<IOEvent, DateTime<Utc>>,
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

        let metadata: device::DeviceMetadata =
            device::DeviceMetadata::new( name, version_id, sensor_id, kind, );

        let log = <IOEvent>::container();

        MockPhSensor { metadata, log }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
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

    /// Call `get_event` and add to log
    /// listeners would be asynchronously called here
    fn poll(&mut self, time: DateTime<Utc>) -> crate::errors::Result<()> {
        self.log.add(time, self.get_event(time))
    }
}

impl From<device::DeviceMetadata> for MockPhSensor {
    fn from(metadata: device::DeviceMetadata) -> Self {
        Self { metadata, log: <IOEvent>::container() }
    }
}