/// Provide Low-level Device Functionality
use crate::io::metadata::DeviceMetadata;

/// Basic interface for GPIO device metadata
pub trait Device {
    fn get_metadata(&self) -> &DeviceMetadata;
    fn name(&self) -> String;
    fn id(&self) -> i32;
}
