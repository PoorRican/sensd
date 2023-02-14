use crate::io;
use crate::io::IdType;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Encapsulation of individual device metadata
///
/// This struct stores information about a device, including its name, version ID, sensor ID,
/// kind, minimum and maximum value, and resolution.
///
/// # Example
///
/// ```
/// let name = "Device".to_string();
/// let id = 1;
/// let kind = crate::io::IOKind::PH;
///
/// let info = crate::DeviceInfo::new(name, id, kind, None);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceMetadata {
    pub name: String,
    pub id: IdType,
    pub kind: io::IOKind,
    pub direction: io::IODirection,
}

impl DeviceMetadata {
    /// Creates a new instance of `DeviceInfo`
    ///
    /// # Arguments
    ///
    /// * `name`: name of device
    /// * `id`: ID of the device (user provided)
    /// * `kind`: IOKind representing device type
    /// * `direction`: IODirection representing device type
    ///
    /// # Returns
    ///
    /// A new instance with given specified parameters
    pub fn new(name: String, id: IdType, kind: io::IOKind, direction: io::IODirection) -> Self {
        DeviceMetadata {
            name,
            id,
            kind,
            direction,
        }
    }
}

impl std::fmt::Display for DeviceMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device Info {{ Kind: {}, Direction: {} }}", self.kind, self.direction,)
    }
}
