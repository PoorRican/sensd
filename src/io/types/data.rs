use crate::io::{IOKind, RawValue};
use serde::{Deserialize, Serialize};

/// Encapsulates I/O data. Provides a unified data type for returning data.
/// Eventually Direction will be added as a value.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct IOData {
    pub kind: IOKind,
    pub value: RawValue,
}

impl IOData {
    pub fn new(kind: IOKind, value: RawValue) -> Self {
        let kind = kind;
        Self { kind, value }
    }
}
