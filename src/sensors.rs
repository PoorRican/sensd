pub trait SensorValue {
    fn read<T>(&self) -> T;
}

pub trait CalibratedSensor {
    fn calibrate(&self);
}

pub enum SensorType {
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

pub struct SensorData<T> {
    kind: SensorType,
    data: T
}

pub struct SensorInfo<T> {
    name: String,
    version_id: i32,
    sensor_id: i32,
    kind: SensorType,

    min_value: T,   // min value (in SI units)
    max_value: T,   // max value (in SI units)
    resolution: T,  // resolution of sensor (in SI units)

    min_delay: i32, // minimum delay between sensing events
}

impl<T> SensorInfo<T> {
    pub fn new<T>(name: String, version_id: i32, sensor_id: i32,
                  kind: SensorType, min_value: T, max_value: T, resolution: T, min_delay: i32) -> Self<T> {
        SensorInfo {
            name, version_id, sensor_id,
            kind, min_value, max_value, resolution, min_delay
        }
    }
}

pub struct SensorEvent<T> {
    version_id: i32,
    sensor_id: i32,
    timestamp: i32,
    data: SensorData<T>,
}

impl SensorEvent<T> {
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
    pub fn create<T>(&info: &SensorInfo<T>, timestamp: i32, value: T) -> Self {
        let version_id = info.version_id;
        let sensor_id = info.sensor_id;
        let data = SensorData {
            kind: info.kind,
            data: value
        };
        SensorEvent {
            version_id,
            sensor_id,
            timestamp,
            data
        }
    }
}
