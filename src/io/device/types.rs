pub use crate::helpers::Def;
pub use crate::io::{
    DeviceWrapper, DeviceTraits, Device,
    GenericInput, GenericOutput,
    IdType, IOKind, IODirection,
};
pub use std::sync::{Arc, Mutex};

pub enum DeviceType {
    Input(GenericInput),
    Output(GenericOutput),
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

    fn direction(&self) -> IODirection {
        match self {
            Self::Output(inner) => inner.direction(),
            Self::Input(inner) => inner.direction(),
        }
    }
}
