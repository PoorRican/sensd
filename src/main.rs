extern crate chrono;
extern crate serde;

mod errors;
mod io;
mod polling;
mod sensors;
mod settings;
mod units;
mod storage;

use std::sync::Arc;

use crate::storage::MappedCollection;
use crate::io::Device;
use crate::errors::Result;
use crate::polling::PollGroup;
use crate::sensors::ph::MockPhSensor;
use crate::settings::Settings;
use crate::storage::Persistent;

fn main() -> Result<()> {
    // # Load Settings
    let settings: Arc<Settings> = Arc::new(Settings::initialize());

    // # Setup Poller
    let mut poller: PollGroup<i32> = PollGroup::new( "main", settings);

    let s0 = MockPhSensor::new("test name".to_string(), 0);
    let s1 = MockPhSensor::new("second sensor".to_string(), 1);

    let sensors = [s0, s1];
    for sensor in sensors {
        poller.sensors.add(sensor.id(), sensor.boxed())?;
    }

    loop {
        match poller.poll() {_ => ()};
        std::thread::sleep(std::time::Duration::from_secs(1));
        // match poller.save() {
        //     Ok(_) => (),
        //     Err(t) => return Err(t)
        // };
    }

}
