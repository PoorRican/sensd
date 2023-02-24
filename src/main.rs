extern crate chrono;
extern crate serde;

mod action;
mod builders;
mod errors;
mod helpers;
mod io;
mod settings;
mod storage;
mod units;

use std::sync::Arc;

use crate::action::{BaseCommandFactory, Comparison, SimpleNotifier, GPIOCommand, IOCommand};
use crate::builders::ActionBuilder;
use crate::errors::Result;
use crate::io::{IODirection, IOKind, IOType};
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
        ("test name", 0, IOKind::PH, IODirection::Input, IOCommand::Input(move || IOType::Float(1.2))),
        ("second sensor", 1, IOKind::Flow, IODirection::Input, IOCommand::Input(move || IOType::Float(0.5))),
    ];
    poller.add_devices(&config).unwrap();

    // build subscribers/commands
    println!("\nBuilding subscribers ...");

    for (id, input) in poller.inputs.iter() {
        println!("\n- Setting up builder ...");

        let mut builder = ActionBuilder::new(input.clone())?;

        println!("- Initializing subscriber ...");

        let name = format!("Subscriber for Input:{}", id);
        let threshold = IOType::Float(1.0);
        let trigger = Comparison::GT;
        let factory: BaseCommandFactory =
            |value, threshold| SimpleNotifier::command(format!("{} exceeded {}", value, threshold));
        builder.add_threshold(&name, threshold, trigger, factory);
    }

    println!("\n... Finished building\n");

    // main event loop
    println!("... Beginning polling ...\n");
    loop {
        let polled = poller.poll();
        match polled {
            Ok(_) => match poller.save(&None) {
                Ok(_) => (),
                Err(t) => {
                    return Err(t);
                }
            },
            _ => (),
        };
        std::thread::sleep(FREQUENCY);
    }
}
