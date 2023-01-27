extern crate chrono;

mod container;
mod device;
mod io;
mod settings;
mod sensors;
mod units;

use chrono::Duration;
use std::collections::HashMap;
use std::time;

use crate::container::Collection;
use crate::device::Device;
use crate::sensors::ph::MockPhSensor;
use crate::settings::Settings;


fn main() {
    let s1 = MockPhSensor::new(
        "test name".to_string(),
        0,
        chrono::Duration::seconds(5),
    );
    let mut container = container::Container::<Box<dyn device::Sensor<units::Ph>>, i32>::new();
    container.add(0, Box::new(s1.clone()));
    dbg!(s1);
}
