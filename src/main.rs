extern crate chrono;
extern crate serde;

mod errors;
mod helpers;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::{Arc};
use crate::helpers::{Deferrable};

use crate::errors::Result;
use crate::io::{Direction, IOKind, ThresholdNotifier};
use crate::settings::Settings;
use crate::storage::{Persistent, PollGroup};

/// Operating frequency
/// Allows for operations to occur at any multiple of once per second
const FREQUENCY: std::time::Duration = std::time::Duration::from_secs(1);

/// Load settings and setup `PollGroup`
/// # Args
/// name - Name to be converted to string
fn init(name: &str) -> PollGroup {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    println!("Initialized settings");

    let group = PollGroup::new(name.clone(), Some(settings));
    println!("Initialized poll group: \"{}\"", name);
    group
}

fn main() -> Result<()> {
    let mut poller = init("main");
    let config = vec![("test name", 0, IOKind::PH), ("second sensor", 1, IOKind::Flow)];
    poller.add_inputs(&config).unwrap();

    // build subscribers/commands
    println!("\nBuilding subscribers ...");
    for (_, &ref sensor) in poller.inputs.iter() {
        let notifier = ThresholdNotifier::new(
            1.0,
            sensor.clone(),
            Direction::Above
        );
        dbg!(notifier.clone());
        let deferred = notifier.deferred();
        sensor.try_lock().unwrap().subscribe(deferred);
    };
    println!("... Finished building\n");

    // main event loop
    println!("... Beginning polling ...\n");
    loop {
        let polled = poller.poll();
        println!("...polled");
        match polled {
            Ok(_) => match poller.save(&None) {
                Ok(_) => (),
                Err(t) => {
                    dbg!("Error");
                    return Err(t)
                },
            },
            _ => (),
        };
        std::thread::sleep(FREQUENCY);
    }
}
