use crate::io::{
    IOKind, IOType,
};
use serde::{Deserialize, Serialize};

/// Encapsulates I/O data. Provides a unified data type for returning data.
/// Eventually Direction will be added as a value.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct IOData {
    pub kind: IOKind,
    pub value: IOType,
}
