/// Provide Low-level Device Functionality
use crate::helpers::{Deferred, Deferrable};
use crate::io::{IdTraits, IODirection, IOKind};
use crate::io::metadata::DeviceMetadata;
use crate::storage::OwnedLog;

pub type IdType = u32;

impl IdTraits for IdType {}

/// Basic interface for GPIO device metadata
pub trait Device {
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Deferred<OwnedLog>) -> Self where Self: Sized;

    fn metadata(&self) -> &DeviceMetadata;

    fn name(&self) -> String {
        self.metadata().name.clone()
    }

    fn id(&self) -> IdType {
        self.metadata().id
    }

    fn direction(&self) -> IODirection {
        self.metadata().direction
    }
}
