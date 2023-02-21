//! Low-level type and interface definitions for I/O with the filesystem, memory, and other resources.

use crate::helpers::{Deferred};
use crate::io::{Device, GenericInput, GenericOutput};
use crate::storage::{Container};
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
impl Default for IOType {
    fn default() -> Self {
        IOType::Float(f32::default())
    }
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
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

impl Display for IODirection {
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

impl Display for IOKind {
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

pub trait DeviceWrapper {
    fn is_input(&self) -> bool;
    fn is_output(&self) -> bool;
}

pub trait DeviceTraits {
    fn name(&self) -> String;
    fn id(&self) -> IdType;
    fn kind(&self) -> IOKind;
}

pub enum DeviceType {
    Input(GenericInput),
    Output(GenericOutput)
}
impl DeviceWrapper for DeviceType {
    fn is_input(&self) -> bool {
        match self {
            Self::Input(_) => true,
            Self::Output(_) => false,
        }
    }
    fn is_output(&self) -> bool {
        match self {
            Self::Input(_) => false,
            Self::Output(_) => true,
        }
    }
}
impl DeviceTraits for DeviceType {
    fn name(&self) -> String {
        match self {
            Self::Output(inner) => inner.name(),
            Self::Input(inner) => inner.name(),
        }
    }

    fn id(&self) -> IdType {
        match self {
            Self::Output(inner) => inner.id(),
            Self::Input(inner) => inner.id(),
        }
    }

    fn kind(&self) -> IOKind {
        match self {
            Self::Output(inner) => inner.kind(),
            Self::Input(inner) => inner.kind(),
        }
    }
}

pub type DeferredDevice = Deferred<DeviceType>;
impl DeviceWrapper for DeferredDevice {
    fn is_input(&self) -> bool {
        let binding = self.lock().unwrap();
        binding.is_input()
    }
    fn is_output(&self) -> bool {
        let binding = self.lock().unwrap();
        binding.is_input()
    }
}
impl DeviceTraits for DeferredDevice {
    fn name(&self) -> String {
        self.lock().unwrap().name()
    }

    fn id(&self) -> IdType {
        self.lock().unwrap().id()
    }

    fn kind(&self) -> IOKind {
        self.lock().unwrap().kind()
    }
}

/// Alias for using a deferred devices in `Container`, indexed by `K`
pub type DeviceContainer<K> = Container<DeferredDevice, K>;
