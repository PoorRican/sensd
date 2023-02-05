extern crate chrono;
extern crate serde;

mod errors;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::Arc;

use sensd;
use sensd::errors::Result;
use sensd::settings::Settings;
use sensd::storage::PollGroup;

fn main() -> Result<()> {
    // # Load Settings
    let settings: Arc<Settings> = Arc::new(Settings::initialize());

    // # Setup Poller
    let mut poller: PollGroup = PollGroup::new("main", settings);

    let config = vec![("test name", 0), ("second sensor", 1)];
    for result in poller.add_sensors(config) {
        result.unwrap();
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
