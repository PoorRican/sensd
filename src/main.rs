extern crate chrono;

mod container;
mod device;
mod io;
mod polling;
mod sensors;
mod settings;
mod units;

use chrono::{DateTime, Duration, Utc};

use crate::container::{Collection, Container, Containerized};
use crate::device::Sensor;
use crate::polling::Poller;
use crate::sensors::ph::MockPhSensor;
use crate::settings::Settings;
use crate::units::Ph;

fn main() {
    static SETTINGS: Settings = Settings::initialize();
    unsafe {
        static mut SENSORS: Container<Box<dyn Sensor<Ph>>, i32> = <dyn Sensor<Ph>>::container();
        static mut LOG: Container<io::IOEvent<Ph>, DateTime<Utc>> = <io::IOEvent<Ph>>::container();
        static mut POLLER: Poller<Ph, i32> = Poller::new(
            &settings.interval,
            Utc::now() - settings.interval,
            &mut SENSORS,
            &mut LOG
        );
    }


    let s0 = MockPhSensor::new("test name".to_string(), 0, Duration::seconds(5));
    let s1 = MockPhSensor::new("second sensor".to_string(), 1, Duration::seconds(10));

    container.add(0, Box::new(s0));
    container.add(1, Box::new(s1));

    dbg!(container._inner());

}
