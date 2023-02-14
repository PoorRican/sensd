use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::{Device, IdType, IODirection};
use crate::io::types::{IdTraits, IOData, IOType};
use crate::storage::{Container, Containerized, LogType};

/// Encapsulates `IOData` alongside of timestamp and device data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IOEvent {
    pub id: IdType,
    pub timestamp: DateTime<Utc>,
    pub direction: IODirection,

    #[serde(flatten)]
    pub data: IOData,
}

// TODO: add kind to `IOEvent`
impl IOEvent {
    /// Generate sensor event.
    ///
    /// # Arguments
    ///
    /// * `device`: struct that has implemented the `Device` trait
    /// * `timestamp`: timestamp of event
    /// * `value`: value to include in
    ///
    /// returns: SensorEvent
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn create(device: &(impl Device + ?Sized), timestamp: DateTime<Utc>, value: f64) -> Self {
        let direction = device.direction();
        let info = device.metadata();
        let id = info.id;
        let data = IOData {
            kind: info.kind.clone(),
            data: value,
        };
        IOEvent {
            id,
            timestamp,
            direction,
            data,
        }
    }

    pub fn invert(&self, value: IOType) -> Self {
        let mut inverted = self.clone();
        inverted.data.data = value;
        inverted.direction = match inverted.direction {
            IODirection::Input => IODirection::Output,
            IODirection::Output => IODirection::Input,
        };
        inverted
    }
}

impl IdTraits for DateTime<Utc> {}

/// Return a new instance of `Container` with for storing `IOEvent` which are accessed by `DateTime<Utc>` as keys
impl Containerized<IOEvent, DateTime<Utc>> for IOEvent {
    fn container() -> LogType {
        Container::<IOEvent, DateTime<Utc>>::new()
    }
}
