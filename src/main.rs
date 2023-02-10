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

/// Load settings and setup `PollGroup`
fn init(name: &str) -> PollGroup {
    let settings: Arc<Settings> = Arc::new(Settings::_initialize());

    PollGroup::new(name, settings)
}

fn main() -> Result<()> {
    let mut poller = init("main");
    let config = vec![("test name", 0), ("second sensor", 1)];
    poller._add_sensors(&config).unwrap();

    loop {
        match poller.poll() {
            Ok(_) =>
                match poller.save(&None) {
                    Ok(_) => (),
                    Err(t) => return Err(t)
                },
            _ => (),
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
