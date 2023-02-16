extern crate chrono;
extern crate serde;

mod builders;
mod errors;
mod helpers;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::Arc;

use crate::builders::pubsub_builder;
use crate::errors::Result;
use crate::io::{Comparison, IOKind, IOType, SimpleNotifier};
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
    let config = vec![
        ("test name", 0, IOKind::PH),
        ("second sensor", 1, IOKind::Flow),
    ];
    poller.add_inputs(&config).unwrap();

    // build subscribers/commands
    println!("\nBuilding subscribers ...");
    for (id, input) in poller.inputs.iter() {
        let name = format!("Subscriber for Input:{}", id);
        pubsub_builder(
            input.clone(),
            name,
            IOType::Float(1.0),
            Comparison::GT,
            (|value, threshold| {
                SimpleNotifier::command(format!("{} exceeded {}", value, threshold))
            }),
        )
    }
    println!("... Finished building\n");

    // main event loop
    println!("... Beginning polling ...\n");
    loop {
        let polled = poller.poll();
        match polled {
            Ok(_) => match poller.save(&None) {
                Ok(_) => (),
                Err(t) => {
                    dbg!("Error");
                    return Err(t);
                }
            },
            _ => (),
        };
        std::thread::sleep(FREQUENCY);
    }
}
