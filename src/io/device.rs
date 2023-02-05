/// Provide Low-level Device Functionality
use crate::io::metadata::DeviceMetadata;

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
