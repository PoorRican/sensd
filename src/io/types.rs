use crate::helpers::Deferred;
use crate::io::{Device, Input, Output};
use crate::storage::Container;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::hash::Hash;

/// Type returned by input devices.
///
/// # Notes
/// Eventually this will be converted to an enum for storing type.
/// This is just a placeholder to establish throughout the codebase.
pub type IOType = f64;

/// Traits required to be implemented for a type to be usable as an `id`
pub trait IdTraits: Eq + Hash + Default + Serialize {}

/// Type used to index and identify I/O device objects
///
/// # Notes
/// Eventually this will be converted to a tuple for storing complex data.
/// This is just a placeholder to establish throughout the codebase.
pub type IdType = u32;

impl IdTraits for IdType {}

/// Encapsulates I/O data. Provides a unified data type for returning data.
/// Eventually Direction will be added as a value.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct IOData {
    pub kind: IOKind,
    pub data: IOType,
}

/// Enum used to classify direction of data flow in relation to system.
///
/// Input objects generate data from the outside world;
/// output objects accept data, and manipulate the outside.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
pub enum IODirection {
    #[default]
    Input,
    Output,
}

impl std::fmt::Display for IODirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IODirection::Input => "Input",
            IODirection::Output => "Output",
        };
        write!(f, "{}", name)
    }
}

/// Defines I/O type.
/// Intended to allow differentiation and classification.
/// More may be appended as time goes on.
/// This might be expanded to multiple levels of organization.
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

pub struct DeviceType(Box<dyn Device>);

/// hack to work around using `Box<dyn Input + Device
pub type InputType = Box<dyn Input>;
pub type OutputType = Box<dyn Output>;

/// Alias for using a deferred `InputType` in `Container`, indexed by `K`
pub type InputContainer<K> = Container<Deferred<InputType>, K>;