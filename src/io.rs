/// Encapsulate IO for devices
use std::fmt::Formatter;

use chrono::{Utc, DateTime};
use crate::device;
use crate::container::{Container, Containerized};

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

// TODO: add kind to `IOEvent`
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
}


/// Return a new instance of `Container` with for storing `IOEvent<T>` which are accessed by `DateTime<Utc>` as keys
impl<T> Containerized<IOEvent<T>, DateTime<Utc>> for IOEvent<T>
    where T: std::fmt::Debug
{
    fn container() -> Container<IOEvent<T>, DateTime<Utc>> {
        Container::<IOEvent<T>, DateTime<Utc>>::new()
    }
}
