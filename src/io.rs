/// Encapsulate IO for devices

use chrono::{Local, DateTime, Date};
use crate::device;

/// Defines sensor type. Used to classify data along with `IOData`
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
    timestamp: DateTime<Local>,
    data: IOData<T>,
}

impl IOEvent<T> {
    /// Generate sensor event.
    ///
    /// # Arguments
    ///
    /// * `&info`: Sensor info
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
    pub fn create<T>(&info: &device::DeviceInfo<T>, timestamp: DateTime<Local>, value: T) -> Self {
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = IOData {
            kind: info.kind,
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

