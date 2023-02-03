use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use crate::io;

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMetadata {
    pub name: String,
    pub version_id: i32,
    pub sensor_id: i32,
    pub kind: io::IOKind,
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
    pub fn new(
        name: String,
        version_id: i32,
        sensor_id: i32,
        kind: io::IOKind,
    ) -> Self {
        DeviceMetadata {
            name,
            version_id,
            sensor_id,
            kind,
        }
    }
}

impl std::fmt::Display for DeviceMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device Info {{ Kind: {} }}",
            self.kind,
        )
    }
}
