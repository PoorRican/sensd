use crate::io::metadata::DeviceMetadata;
/// Provide Low-level Device Functionality
use crate::io::IdTraits;

pub type IdType = u32;

impl IdTraits for IdType {}

/// Basic interface for GPIO device metadata
pub trait Device {
    fn metadata(&self) -> &DeviceMetadata;

    fn name(&self) -> String {
        self.metadata().name.clone()
    }

    fn id(&self) -> IdType {
        self.metadata().sensor_id
    }
}
