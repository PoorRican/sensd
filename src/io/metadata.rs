use crate::io;
use crate::io::device::IdType;
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
/// let name = "PH Sensor".to_string();
/// let version_id = 0;
/// let sensor_id = 1;
/// let kind = crate::io::IOKind::PH;
/// let min_value = 0.0;
/// let max_value = 14.0;
/// let resolution = 0.1;
///
/// let info = crate::DeviceInfo::new(name, version_id, sensor_id, kind, min_value, max_value, resolution);
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
    /// * `version_id`: driver or interface version ID
    /// * `sensor_id`: sensor ID of the device (should be arbitrary)
    /// * `kind`: IOKind representing device type
    /// * `min_value`: measurable or theoretical minimum value (in SI units)
    /// * `max_value`: measurable or theoretical maximum value (in SI units)
    /// * `resolution`: measurable resolution of the device
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
