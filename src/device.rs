/// Provide Low-level Device Functionality

use crate::io;


pub trait Readable {
    fn read<T>(&self) -> T;
}


/// Represents a sensor that requires calibration
pub trait Calibratable {
    fn calibrate(&self) -> Result<T, E>;
}


/// Encapsulates individual device info
/// Meant to used as a struct attribute via `new()`
pub struct DeviceInfo<T> {
    pub name: String,
    pub version_id: i32,
    pub sensor_id: i32,
    pub kind: io::IOKind,

    min_value: T,   // min value (in SI units)
    max_value: T,   // max value (in SI units)
    resolution: T,  // resolution of sensor (in SI units)

    min_delay: u16, // minimum delay between sensing events
}


impl<T> DeviceInfo<T> {
    pub fn new<T>(name: String, version_id: i32, sensor_id: i32,
                  kind: io::IOKind, min_value: T, max_value: T, resolution: T, min_delay: i32) -> Self<T> {
        DeviceInfo {
            name, version_id, sensor_id,
            kind, min_value, max_value, resolution, min_delay
        }
    }
}

