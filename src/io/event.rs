use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::{DeviceMetadata, IOData, IODirection, IdTraits, IdType, RawValue};

/// Encapsulates [`IOData`] alongside of timestamp and device data
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct IOEvent {
    pub id: IdType,
    pub timestamp: DateTime<Utc>,
    pub direction: IODirection,

    #[serde(flatten)]
    pub data: IOData,
}

impl IOEvent {
    /// Constructor for [`IOEvent`]
    ///
    /// # Arguments
    ///
    /// * `metadata`: device metadata
    /// * `timestamp`: timestamp of event
    /// * `value`: value to include in
    ///
    /// # Returns
    /// `IOEvent` based on device metadata, timestamp, and value
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(metadata: &DeviceMetadata, timestamp: DateTime<Utc>, value: RawValue) -> Self {
        let direction = metadata.direction;
        let id = metadata.id;
        let data = IOData::new(metadata.kind, value);
        IOEvent {
            id,
            timestamp,
            direction,
            data,
        }
    }
}

impl IdTraits for DateTime<Utc> {}
