/// Encapsulate IO for devices
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::hash::Hash;

mod calibrated;
mod device;
mod event;
mod input;
mod metadata;
mod sensors;

use crate::helpers::Deferred;
pub use calibrated::Calibrated;
pub use device::*;
pub use event::IOEvent;
pub use input::Input;
pub use metadata::DeviceMetadata;
pub use sensors::*;

use crate::storage::Container;

/// Defines sensor type. Used to classify data along with `IOData`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum IOKind {
    #[default]
    Unassigned,
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
            IOKind::Unassigned => "Unassigned",
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

/// Traits required to be implemented for a type to be usable as an `id`
pub trait IdTraits: Eq + Hash + Default + Serialize {}

pub trait InputDevice: Input + Device {}

pub struct DeviceType(Box<dyn Device>);

/// Facade for input objects
pub struct InputType(Box<dyn InputDevice>);
impl Device for InputType {
    fn metadata(&self) -> &DeviceMetadata {
        &self.0.metadata()
    }
}
impl Input for InputType {
    /// facade for input device implementation
    /// Should eventually return `Result<f64>`
    fn read(&self) -> f64 {
        self.0.read()
    }

    /// facade for polling
    fn poll(&mut self, time: DateTime<Utc>) -> crate::errors::Result<()> {
        self.0.poll(time)
    }
}

pub type InputContainer<K> = Container<Deferred<InputType>, K>;
