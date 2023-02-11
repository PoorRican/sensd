use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, DeviceMetadata, IOKind, IdType, Input, InputDevice, InputType};
use crate::storage::{MappedCollection, OwnedLog};

pub trait Sensor: Default + InputDevice + Deferrable {
    fn new(name: String, sensor_id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self;
}

use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};


#[derive(Debug, Default)]
pub struct GenericSensor {
    metadata: DeviceMetadata,
    pub log: Deferred<OwnedLog>,
}

/** Represents a mock pH sensor.
 */
impl Sensor for GenericSensor {
    /// Creates a mock ph sensor which returns random values
    ///
    /// # Arguments
    ///
    /// * `name`: arbitrary name of sensor
    /// * `sensor_id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new(name: String, sensor_id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self {
        let kind = kind.unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name, sensor_id, kind);

        GenericSensor { metadata, log }
    }
}

impl Deferrable for GenericSensor {
    type Inner = InputType;
    /// Return wrapped Sensor in
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(InputType(Box::new(self))))
    }
}

// Implement traits
impl Device for GenericSensor {
    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
}

impl Input for GenericSensor {
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

impl InputDevice for GenericSensor {}
