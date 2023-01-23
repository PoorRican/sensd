/// Encapsulate IO for devices

use chrono::{Utc, DateTime};
use crate::device;

/// Defines sensor type. Used to classify data along with `IOData`.
#[derive(Debug, Clone, Copy)]
pub enum IOKind {
    Light,
    Pressure,
    Proximity,
    RotationVector,
    RelativeHumidity,
    AmbientTemperature,
    Voltage,
    Current,
    Color,
    TVOC,
    VocIndex,
    NoxIndex,
    FLOW,
    EC,
    PH,
}

// TODO: enum for `IODirection` when implementing control system

/// Encapsulates sensor data. Provides a unified data type for returning data.
pub struct IOData<T> {
    kind: IOKind,
    data: T
}

impl<T> IOData<T> {
    pub fn new<T>(kind: IOKind, data: T) -> Self {
        IOData { kind, data }
    }
}

/// Encapsulates `IOData` alongside of timestamp and device data
pub struct IOEvent<T> {
    version_id: i32,
    sensor_id: i32,
    timestamp: DateTime<Utc>,
    data: IOData<T>,
}

impl IOEvent<T> {
    /// Generate sensor event.
    ///
    /// # Arguments
    ///
    /// * `device`: struct that has implemented the `Device` trait
    /// * `timestamp`: timestamp of event
    /// * `value`: value to include in
    ///
    /// returns: SensorEvent<T>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn create<T>(&device: &impl device::Device<T>, timestamp: DateTime<Utc>, value: T) -> Self {
        let info = &device.get_info();
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = IOData {
            kind: info.kind.clone(),
            data: value
        };
        IOEvent {
            version_id,
            sensor_id,
            timestamp,
            data
        }
    }
}

