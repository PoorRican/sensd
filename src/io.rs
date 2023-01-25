/// Encapsulate IO for devices
use polars::prelude::*;
use polars::datatypes::DataType::Datetime;

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
    pub kind: IOKind,
    pub data: T
}

/// Encapsulates `IOData` alongside of timestamp and device data
pub struct IOEvent<T> {
    pub version_id: i32,
    pub sensor_id: i32,
    pub timestamp: DateTime<Utc>,
    pub data: IOData<T>,
}

// TODO: create a function that converts `IOEvent` to DataFrame

impl<T> IOEvent<T> {
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
    pub fn create( device: &impl device::Device<T>, timestamp: DateTime<Utc>, value: T ) -> Self {
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

    pub fn schema() -> Schema {
        let version_id = Field::new("version_id", DataType::Int32);
        let sensor_id = Field::new("sensor_id", DataType::Int32);
        let timestamp = Field::new("timestamp", DataType::Datetime(TimeUnit::Nanoseconds, None));
        let data = Field::new("data", DataType::Float64);

        Schema::from(vec![version_id, sensor_id, timestamp, data].into_iter())
    }
}
