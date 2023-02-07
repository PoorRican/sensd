use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};
use crate::io::device::IdType;

use crate::io::metadata::DeviceMetadata;
use crate::io::{Device, Input, IOKind};
use crate::storage::{MappedCollection, LogType};

#[derive(Debug)]
pub struct MockPhSensor {
    metadata: DeviceMetadata,
    pub log: Arc<Mutex<LogType>>,
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
    pub fn new(name: String, sensor_id: IdType, log: Arc<Mutex<LogType>>) -> Self {
        let version_id = 0;
        let kind = IOKind::PH;

        let metadata: DeviceMetadata = DeviceMetadata::new(name, version_id, sensor_id, kind);

        MockPhSensor { metadata, log }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

// Implement traits
impl Device for MockPhSensor {
    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
}

impl Input for MockPhSensor {
    /// Return a mock value
    fn read(&self) -> f64 {
        1.2
    }

    /// Call `get_event` and add to log
    /// listeners would be asynchronously called here
    fn poll(&mut self, time: DateTime<Utc>) -> crate::errors::Result<()> {
        self.log.lock().unwrap().add(time, self.get_event(time))
    }
}
