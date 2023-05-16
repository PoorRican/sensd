use crate::io;
use crate::io::{IdType, IOKind, IODirection};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Encapsulate device metadata
///
/// This struct stores information about a physical or abstract device, including a user provided name, ID,
/// the kind of device, and the dataflow direction (defaults to input). In future releases, the included data
/// must be minimal and remain universal and agnostic to device type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceMetadata {
    /// User given name of device
    pub name: String,

    /// User given device id
    pub id: IdType,

    /// Sensor/device type
    pub kind: IOKind,

    /// I/O direction
    pub direction: IODirection,
}

impl DeviceMetadata {
    /// Creates a new instance of `DeviceInfo`
    ///
    /// # Arguments
    ///
    /// - `name`: name of device
    /// - `id`: ID of the device (user provided)
    /// - `kind`: IOKind representing device type
    /// - `direction`: IODirection representing device type
    ///
    /// # Returns
    ///
    /// A new [`DeviceMetadata`] instance with given parameters
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::{IOKind, DeviceMetadata, IODirection, IdType};
    ///
    /// let name = "Device";
    /// let id = 1;
    /// let kind = IOKind::PH;
    /// let direction = IODirection::default();
    ///
    /// let metadata = DeviceMetadata::new(name, id, kind, direction);
    ///
    /// assert_eq!(metadata.name, name);
    /// assert_eq!(metadata.id, id);
    /// assert_eq!(metadata.kind, kind);
    /// assert_eq!(metadata.direction, direction);
    /// ```
    pub fn new<N>(name: N, id: IdType, kind: io::IOKind, direction: io::IODirection) -> Self
    where
        N: Into<String>,
    {
        DeviceMetadata {
            name: name.into(),
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

#[cfg(test)]
mod tests {
    use crate::io::{DeviceMetadata, IODirection, IOKind};

    #[test]
    /// Test that constructor accepts `name` parameter as `&str` or `String`
    fn new_name_parameter() {
        DeviceMetadata::new("as &str", 0, IOKind::default(), IODirection::default());
        DeviceMetadata::new(String::from("as String"), 0, IOKind::default(), IODirection::default());
    }
}