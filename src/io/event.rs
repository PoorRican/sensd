use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::types::{IOData, IOType, IdTraits};
use crate::io::{Device, IODirection, IdType};
use crate::storage::{Container, Containerized, LogType};

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

    /// Invert a copy of existing `IOEvent` and inject a new value.
    /// This should be used for converting an `IOEvent` from input to output.
    pub fn invert(&self, value: Option<IOType>) -> Self {
        let mut inverted = self.clone();
        if value.is_some() {
            inverted.data.value = value.unwrap();
        }
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
