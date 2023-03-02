use crate::io;
use crate::io::IdType;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Encapsulate device metadata
///
/// This struct stores information about a physical or abstract device, including a user provided name, ID,
/// the kind of device, and the dataflow direction (defaults to input). In future releases, the included data
/// must be minimal and remain universal and agnostic to device type.
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
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::{IOKind, DeviceMetadata, IODirection};
    ///
    /// let name = "Device".to_string();
    /// let id = 1;
    /// let kind = IOKind::PH;
    /// let direction = IODirection::default();
    ///
    /// let info = DeviceMetadata::new(name, id, kind, direction);
    /// ```
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
        write!(
            f,
            "Device Info {{ Kind: {}, Direction: {} }}",
            self.kind, self.direction,
        )
    }
}
