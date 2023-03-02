use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::types::{IOData, IOType, IdTraits};
use crate::io::{Device, IODirection, IdType};

/// Encapsulates `IOData` alongside of timestamp and device data
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct IOEvent {
    pub id: IdType,
    pub timestamp: DateTime<Utc>,
    pub direction: IODirection,

    #[serde(flatten)]
    pub data: IOData,
}

// TODO: add kind to `IOEvent`
impl IOEvent {
    /// Generate I/O event.
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
    pub fn generate(device: &(impl Device + ?Sized), timestamp: DateTime<Utc>, value: IOType) -> Self {
        let direction = device.direction();
        let info = device.metadata();
        let id = info.id;
        let data = IOData {
            kind: info.kind.clone(),
            value,
        };
        IOEvent {
            id,
            timestamp,
            direction,
            data,
        }
    }
}

impl IdTraits for DateTime<Utc> {}
