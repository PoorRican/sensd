pub mod ph;


pub trait Readable {
    fn read<T>(&self) -> T;
}

/// Represents a sensor that requires calibration
pub trait Calibratable {
    fn calibrate(&self);
}

/// Defines sensor type. Used to classify data along with `IOData`
pub enum IOKind {
    Light,
    Pressure,
    Proximity,
    RotationVector,
    RelativeHumidity,
    AmbientTemperature,
    Voltage,
    Current,
    Color,
    TVOC,
    VocIndex,
    NoxIndex,
    FLOW,
    EC,
    PH,
}

// TODO: enum for `IODirection` when implementing control system

/// Encapsulates sensor data. Provides a unified data type for returning data.
pub struct IOData<T> {
    kind: IOKind,
    data: T
}

/// Encapsulates individual device info
/// Meant to used as a struct attribute via `new()`
pub struct DeviceInfo<T> {
    name: String,
    version_id: i32,
    sensor_id: i32,
    kind: IOKind,

    min_value: T,   // min value (in SI units)
    max_value: T,   // max value (in SI units)
    resolution: T,  // resolution of sensor (in SI units)

    min_delay: i32, // minimum delay between sensing events
}

impl<T> DeviceInfo<T> {
    pub fn new<T>(name: String, version_id: i32, sensor_id: i32,
                  kind: IOKind, min_value: T, max_value: T, resolution: T, min_delay: i32) -> Self<T> {
        SensorInfo {
            name, version_id, sensor_id,
            kind, min_value, max_value, resolution, min_delay
        }
    }
}

pub struct IOEvent<T> {
    version_id: i32,
    sensor_id: i32,
    timestamp: i32,
    data: IOData<T>,
}

impl IOEvent<T> {
    /// Generate sensor event.
    ///
    /// # Arguments
    ///
    /// * `&info`: Sensor info
    /// * `timestamp`: timestamp of event
    /// * `value`: value to include in
    ///
    /// returns: SensorEvent<T>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn create<T>(&info: &DeviceInfo<T>, timestamp: i32, value: T) -> Self {
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = IOData {
            kind: info.kind,
            data: value
        };
        IOEvent {
            version_id,
            sensor_id,
            timestamp,
            data
        }
    }
}
