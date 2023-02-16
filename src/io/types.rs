//! Low-level type and interface definitions for I/O with the filesystem, memory, and other resources.

use crate::helpers::Deferred;
use crate::io::{Input, Output};
use crate::storage::Container;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

/// Type used for passing between IO abstractions.
///
/// An enum is used to avoid defining a generic `IOEvent` which cannot be
/// stored heterogeneously alongside differing types.
///
/// # Notes
/// The implemented types have been chosen as a good fit for GPIO. However,
/// if a type is needed that is not here, feel free to initiate a pull request.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum IOType {
    Binary(bool),
    PosInt8(u8),
    Int8(i8),
    PosInt(u32),
    Int(i32),
    Float(f32),
}

impl Display for IOType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Binary(val) => {
               if *val {
                   "true"
               } else {
                   "false"
               }.to_string()
            },
            Self::PosInt8(val) => val.to_string(),
            Self::Int8(val) => val.to_string(),
            Self::PosInt(val) => val.to_string(),
            Self::Int(val) => val.to_string(),
            Self::Float(val) => val.to_string(),
        })
    }
}

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
    pub value: IOType,
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

/// hack to work around using `Box<dyn Input + Device
pub type InputType = Box<dyn Input>;
pub type OutputType = Box<dyn Output>;

/// Alias for using a deferred `InputType` in `Container`, indexed by `K`
pub type InputContainer<K> = Container<Deferred<InputType>, K>;
