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
    /// let metadata = DeviceMetadata::with_name(name, id, direction);
    ///
    /// assert_eq!(metadata.name, name);
    /// assert_eq!(metadata.id, id);
    /// assert_eq!(metadata.kind, IOKind::default());
    /// assert_eq!(metadata.direction, direction);
    /// ```
    pub fn with_name<N>(name: N, id: IdType, direction: IODirection) -> Self
    where
        N: Into<String>,
    {
        DeviceMetadata {
            name: name.into(),
            id,
            kind: IOKind::default(),
            direction,
        }
    }

    pub fn kind(mut self, kind: IOKind) -> Self {
        self.kind = kind;
        self
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
        DeviceMetadata::with_name("as &str", 0, IODirection::default());
        DeviceMetadata::with_name(String::from("as String"), 0, IODirection::default());
    }

    #[test]
    fn assert_kind_default() {
        let meta = DeviceMetadata::with_name("", 0, IODirection::default());
        assert_eq!(IOKind::default(), meta.kind);
    }

    #[test]
    fn test_kind_setter() {
        let expected = IOKind::EC;
        let meta =
            DeviceMetadata::with_name("", 0, IODirection::default())
                .kind(expected);
        assert_eq!(expected, meta.kind);
    }
}