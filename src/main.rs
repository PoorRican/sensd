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
use crate::storage::{Persistent, PollGroup};

/// Load settings and setup `PollGroup`
/// # Args
/// name - Name to be converted to string
fn init(name: &str) -> PollGroup {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());

    PollGroup::new(name, Some(settings))
}

fn main() -> Result<()> {
    let mut poller = init("main");
    let config = vec![("test name", 0), ("second sensor", 1)];
    poller.add_inputs(&config).unwrap();

    loop {
        match poller.poll() {
            Ok(_) => match poller.save(&None) {
                Ok(_) => (),
                Err(t) => return Err(t),
            },
            _ => (),
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
