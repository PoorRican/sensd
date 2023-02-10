use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

use crate::io::{Device, Input, IOKind, IdType, DeviceMetadata, InputType, InputDevice, DeviceType};
use crate::helpers::Deferred;
use crate::storage::{MappedCollection, LogType, OwnedLog};

#[derive(Debug)]
pub struct MockPhSensor {
    metadata: DeviceMetadata,
    pub log: Deferred<OwnedLog>,
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
    pub fn new(name: String, sensor_id: IdType, log: Deferred<OwnedLog>) -> Self {
        let version_id = 0;
        let kind = IOKind::PH;

        let metadata: DeviceMetadata = DeviceMetadata::new(name, version_id, sensor_id, kind);

        MockPhSensor { metadata, log }
    }

    pub fn deferred(self) -> Deferred<InputType> {
        Arc::new(Mutex::new(InputType(Box::new(self))))
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
        self.log.lock().unwrap().push(time, self.get_event(time))
    }
}

impl InputDevice for MockPhSensor {}
