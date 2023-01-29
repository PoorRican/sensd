#[macro_use]
extern crate chrono;
extern crate serde;

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
use crate::polling::PollGroup;
use crate::sensors::ph::MockPhSensor;
use crate::settings::Settings;
use crate::units::Ph;

fn main() {
    /// # Load Settings
    let settings: Settings = Settings::initialize();

    /// # Setup Poller
    let mut poller: PollGroup<Ph, i32> = PollGroup::new(settings.interval, Utc::now() - settings.interval);

    let s0 = MockPhSensor::new("test name".to_string(), 0, Duration::seconds(5));
    let s1 = MockPhSensor::new("second sensor".to_string(), 1, Duration::seconds(10));

    poller.sensors.add(0, Box::new(s0));
    poller.sensors.add(1, Box::new(s1));

    loop {
        poller.poll();
        std::thread::sleep(std::time::Duration::from_secs(1));
        dbg!(&poller.log);
    }

}
