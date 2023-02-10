extern crate chrono;
extern crate serde;

mod errors;
mod helpers;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::Arc;

use crate::errors::Result;
use crate::settings::Settings;
use crate::storage::{PollGroup, Persistent};

fn main() -> Result<()> {
    // # Load Settings
    let settings: Arc<Settings> = Arc::new(Settings::_initialize());

    // # Setup Poller
    let mut poller: PollGroup = PollGroup::new("main", settings);

    let config = vec![("test name", 0), ("second sensor", 1)];
    poller._add_sensors(&config).unwrap();

    loop {
        match poller.poll() {
            _ => (),
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
        match poller.save(&None) {
            Ok(_) => (),
            Err(t) => return Err(t)
        };
    }
}
