use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};

use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, DeviceMetadata, IdType, Input, IODirection, IOKind, Publisher, SubscriberStrategy};
use crate::io::types::IOType;
use crate::io::types::InputType;
use crate::storage::{MappedCollection, OwnedLog};

#[derive(Default)]
pub struct GenericSensor {
    metadata: DeviceMetadata,
    pub log: Deferred<OwnedLog>,
    subscribers: Vec<Deferred<Box<dyn SubscriberStrategy>>>,
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
    /// Creates a mock sensor which returns a value
    ///
    /// # Arguments
    ///
    /// * `name`: arbitrary name of sensor
    /// * `id`: arbitrary, numeric ID to differentiate from other sensors
    ///
    /// returns: MockPhSensor
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self where Self: Sized {
        let kind = kind.unwrap_or_default();

        let metadata: DeviceMetadata = DeviceMetadata::new(name, id, kind, IODirection::Input);
        let subscribers = Vec::default();

        GenericSensor { metadata, log, subscribers }
    }

    fn metadata(&self) -> &DeviceMetadata {
        &self.metadata
    }
}

impl Input for GenericSensor {
    /// Return a mock value
    fn rx(&self) -> IOType {
        1.2
    }

    /// Get IOEvent, add to log, and propagate to subscribers
    /// Primary interface method during polling.
    fn read(&mut self, time: DateTime<Utc>) -> Result<()> {
        // get IOEvent
        let event = self.generate_event(time, None);

        // propagate to subscribers
        self.notify(&event);

        // add to log
        let result = self.log.lock().unwrap().push(time, event);

        result
    }
}

impl Publisher for GenericSensor {
    fn subscribers(&mut self) -> &mut [Deferred<Box<dyn SubscriberStrategy>>] {
        &mut self.subscribers
    }

    fn subscribe(&mut self, subscriber: Deferred<Box<dyn SubscriberStrategy>>) {
        self.subscribers.push(subscriber)
    }
}