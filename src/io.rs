/// Encapsulate IO for devices
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

mod calibrated;
mod device;
mod event;
mod input;
mod metadata;
mod sensors;

pub use calibrated::Calibrated;
pub use device::*;
pub use event::IOEvent;
pub use input::Input;
pub use metadata::DeviceMetadata;
pub use sensors::*;

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

pub type LogType = Container<IOEvent, DateTime<Utc>>;
pub type InputType = Box<dyn Input>;
