/// Provide Low-level Device Functionality
use crate::helpers::{Deferred};
use crate::io::{IdTraits, IODirection, IOKind};
use crate::io::metadata::DeviceMetadata;
use crate::storage::OwnedLog;

pub type IdType = u32;

impl IdTraits for IdType {}

/// Defines a minimum interface for interacting with GPIO devices.
///
/// A universal constructor is provided that can be shared between any implementing structs.
/// Additionally, an accessor, `metadata()` is defined to provide for the facade methods to access
/// device name, id, direction, and kind. Therefore, implementing structs shall implement a field
/// `metadata` that is mutably accessed through the reciprocal getter method.
pub trait Device {
    /// Creates a new instance of the device with the given parameters.
    /// `name`: name of device.
    /// `id`: device ID.
    /// `kind`: kind of I/O device. Optional argument.
    /// `log`: Optional deferred owned log for the device.
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self
    where
        Self: Sized;

    /// Returns a reference to the device's metadata
    /// from which information such as name, ID, kind, and I/O direction are inferred.
    fn metadata(&self) -> &DeviceMetadata;

    /// Returns the name of the device.
    fn name(&self) -> String {
        self.metadata().name.clone()
    }

    /// Returns the ID of the device.
    fn id(&self) -> IdType {
        self.metadata().id
    }

    /// Returns the I/O direction of the device.
    fn direction(&self) -> IODirection {
        self.metadata().direction
    }

    /// Returns the type of device as `IOKind`.
    fn kind(&self) -> IOKind {
        self.metadata().kind
    }
}
