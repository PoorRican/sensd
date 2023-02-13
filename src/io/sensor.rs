use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};

use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, DeviceMetadata, IdType, Input, InputDevice, InputType, IOKind,
                Publisher, SubscriberStrategy};
use crate::io::IOType;
use crate::storage::{MappedCollection, OwnedLog};

pub trait Sensor: Default + InputDevice + Deferrable + Publisher {
    fn new(name: String, sensor_id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self;
}

#[derive(Default)]
pub struct GenericSensor {
    metadata: DeviceMetadata,
    pub log: Deferred<OwnedLog>,
    subscribers: Vec<Deferred<Box<dyn SubscriberStrategy>>>,
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
        let subscribers = Vec::default();

        GenericSensor { metadata, log, subscribers }
    }
}

impl Deferrable for GenericSensor {
    type Inner = InputType;
    /// Return wrapped Sensor in
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(Box::new(self)))
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
    fn read(&self) -> IOType {
        1.2
    }

    /// Call `get_event` and add to log
    /// Additionally, data is copied and propagated to subscribers
    fn poll(&mut self, time: DateTime<Utc>) -> Result<()> {
        let event = self.get_event(time, None);
        self.notify(&event);
        let result = self.log.lock().unwrap().push(time, event);
        result
    }
}

impl InputDevice for GenericSensor {}

impl Publisher for GenericSensor {
    fn subscribers(&mut self) -> &mut [Deferred<Box<dyn SubscriberStrategy>>] {
        &mut self.subscribers
    }

    fn subscribe(&mut self, subscriber: Deferred<Box<dyn SubscriberStrategy>>) {
        self.subscribers.push(subscriber)
    }
}