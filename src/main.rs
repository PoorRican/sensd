extern crate chrono;
extern crate serde;

mod errors;
mod io;
mod settings;
mod units;
mod storage;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::storage::{MappedCollection, PollGroup, Persistent};
use crate::io::{Device, LogType, MockPhSensor};
use crate::errors::Result;
use crate::settings::Settings;

fn main() -> Result<()> {
    // # Load Settings
    let settings: Arc<Settings> = Arc::new(Settings::initialize());

    // # Setup Poller
    let mut poller: PollGroup<i32> = PollGroup::new( "main", settings);

    let config = [("test name", 0), ("second sensor", 1)];
    for (name, id) in config {
        let log = Arc::new(Mutex::new(LogType::new()));
        poller.logs.push(log);
        let sensor = MockPhSensor::new(name.to_string(), id, poller.logs.last().unwrap().clone());
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
