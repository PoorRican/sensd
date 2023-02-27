use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::types::{IOData, IOType, IdTraits};
use crate::io::{Device, IODirection, IdType, DeviceMetadata};

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
    pub fn generate(metadata: &DeviceMetadata, timestamp: DateTime<Utc>, value: IOType) -> Self {
        let direction = metadata.direction;
        let id = metadata.id;
        let data = IOData {
            kind: metadata.kind,
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
