use std::constraints::{Bound, Constrained, Range};

use crate::sensors;

/// Abstract pH by constraining float values to 0.0 to 14.0
#[derive(Debug, Clone, Copy, Constrained)]
#[constraint(range(min = 0.0, max = 14.0))]
struct Ph(f64);

impl Ph {
    /// Check constraints before returning value
    ///
    /// # Arguments
    ///
    /// * `val`: a float between 0.0 and 14.0. Method panics if called with invalid values.
    ///
    /// returns: Ph
    ///
    /// # Examples
    ///
    /// ```
    /// Ph::new(3.3)    // intended behaviour
    /// Ph::new(-1.0)   // panics
    /// ```
    pub fn new(val: f64) -> Ph {
        Ph::check_constraints(val);
        Ph(val)
    }
}


pub struct MockPhSensor  {
    info: sensors::SensorInfo<Ph>
}

/** Represents a mock pH sensor.
*/
impl MockPhSensor {
    /// Creates a mock ph sensor which returns random values
    ///
    /// # Arguments
    ///
    /// * `name`: arbitrary name of sensor
    /// * `sensor_id`: arbitrary, numeric ID to differentiate from other sensors
    /// * `min_delay`: minimum delay between sensor reads
    ///
    /// returns: MockPhSensor
    pub fn new(name: String, sensor_id: i32, min_delay: i32) -> Self {
        let version_id = 0;
        let kind = sensors::SensorType::PH;
        let min_value = 0.0;
        let max_value = 14.0;
        let resolution = 0.1;

        let info: sensors::SensorInfo<Ph> = sensors::SensorInfo::new(name, version_id, sensor_id,
                                                                    kind, min_value, max_value, resolution,
                                                                     min_delay);

        MockPhSensor {
            info
        }
    }
}

impl sensor::SensorValue for MockPhSensor {
    /// Return a mock value
    fn read<Ph>(&self) -> sensors::SensorEvent<Ph> {

        Ph::new(1.2)
    }
}