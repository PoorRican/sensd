use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Type used for passing between IO abstractions.
///
/// An enum is used to avoid defining a generic `IOEvent` which cannot be
/// stored heterogeneously alongside differing types.
///
/// # Notes
/// The implemented types have been chosen as a good fit for GPIO. However,
/// if a type is needed that is not here, feel free to initiate a pull request.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RawValue {
    Binary(bool),
    PosInt8(u8),
    Int8(i8),
    PosInt(u32),
    Int(i32),
    Float(f32),
}
impl Default for RawValue {
    fn default() -> Self {
        RawValue::Float(f32::default())
    }
}

impl Display for RawValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Binary(val) => {
                    if *val { "true" } else { "false" }.to_string()
                }
                Self::PosInt8(val) => val.to_string(),
                Self::Int8(val) => val.to_string(),
                Self::PosInt(val) => val.to_string(),
                Self::Int(val) => val.to_string(),
                Self::Float(val) => val.to_string(),
            }
        )
    }
}

