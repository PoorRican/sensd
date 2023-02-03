/// Encapsulate IO for devices
use chrono::{DateTime, Utc};
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

use crate::device;
use crate::storage::{Container, Containerized};

/// Defines sensor type. Used to classify data along with `IOData`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    Flow,
    EC,
    PH,
}

impl std::fmt::Display for IOKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IOKind::Light => "Light",
            IOKind::Pressure => "Pressure",
            IOKind::Proximity => "Proximity",
            IOKind::RotationVector => "Rotation Vector",
            IOKind::RelativeHumidity => "Relative Humidity",
            IOKind::AmbientTemperature => "Ambient Temperature",
            IOKind::Voltage => "Voltage",
            IOKind::Current => "Current",
            IOKind::Color => "Color",
            IOKind::TVOC => "TVOC",
            IOKind::VocIndex => "Voc Index",
            IOKind::NoxIndex => "Nox Index",
            IOKind::Flow => "Flow (liquid)",
            IOKind::EC => "Electrical Conductivity (EC)",
            IOKind::PH => "pH",
        };
        write!(f, "{}", name)
    }
}

// TODO: enum for `IODirection` when implementing control system

/// Encapsulates sensor data. Provides a unified data type for returning data.
#[derive(Debug, Serialize, Deserialize)]
pub struct IOData {
    pub kind: IOKind,
    pub data: f64,
}

/// Encapsulates `IOData` alongside of timestamp and device data
#[derive(Debug, Serialize, Deserialize)]
pub struct IOEvent {
    pub version_id: i32,
    pub sensor_id: i32,
    pub timestamp: DateTime<Utc>,

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
    pub fn create(
        device: &(impl device::Device + ?Sized),
        timestamp: DateTime<Utc>,
        value: f64,
    ) -> Self {
        let info = device.get_metadata();
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = IOData {
            kind: info.kind.clone(),
            data: value,
        };
        IOEvent {
            version_id,
            sensor_id,
            timestamp,
            data,
        }
    }
}

/// Return a new instance of `Container` with for storing `IOEvent` which are accessed by `DateTime<Utc>` as keys
impl Containerized<IOEvent, DateTime<Utc>> for IOEvent
where
{
    fn container() -> Container<IOEvent, DateTime<Utc>> {
        Container::<IOEvent, DateTime<Utc>>::new()
    }
}
