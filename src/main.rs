extern crate chrono;
extern crate serde;

mod errors;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::{Arc, Mutex};

use sensd;
use sensd::errors::Result;
use sensd::io::{Device, LogType, MockPhSensor};
use sensd::settings::Settings;
use sensd::storage::{MappedCollection, PollGroup};

fn main() -> Result<()> {
    // # Load Settings
    let settings: Arc<Settings> = Arc::new(Settings::initialize());

    // # Setup Poller
    let mut poller: PollGroup<i32> = PollGroup::new("main", settings);

    let config = [("test name", 0), ("second sensor", 1)];
    for (name, id) in config {
        // variable allowed to go out-of-scope because `poller` owns reference
        let log = Arc::new(Mutex::new(LogType::new()));
        poller.logs.push(log.clone());

        let sensor = MockPhSensor::new(name.to_string(), id, log.clone());
        poller.sensors.add(sensor.id(), sensor.boxed())?;
    }

    loop {
        match poller.poll() {
            _ => (),
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
        // match poller.save() {
        //     Ok(_) => (),
        //     Err(t) => return Err(t)
        // };
    }
}
