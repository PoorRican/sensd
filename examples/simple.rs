//! A basic example of sensd implementation.
//!
//! # Description 
//!
//! This example displays the use of the "ThresholdAction" subscriber. Two input devices are
//! initalized using the `DeviceLog` builder - both devices return a static float value.
//!
//! # Note
//!
//! ## █▓▒░ Global Variables
//!
//! Take note that the `Group` singleton is not a global mutable static variable as this would
//! require unsafe code.
//!
//! ## █▓▒░ Operating Frequency
//! Device polling is not multi-threaded and the frequency of the event loop is determined by a
//! static frequency `FREQUENCY`. There might be a usecase where frequency needs to be modulated,
//! such as during a control cycle for more accurate control. Reduced polling time might be useful
//! in embedded scenarios requiring both power conservation and accurate control.
extern crate chrono;
extern crate sensd;
extern crate serde;

use std::sync::Arc;

use sensd::action::{Comparison, IOCommand, EvaluationFunction};
use sensd::builders::ActionBuilder;
use sensd::errors::ErrorType;
use sensd::io::{IODirection, IOKind, RawValue};
use sensd::settings::Settings;
use sensd::storage::{Persistent, Group};

/// █▓▒░ Event Loop Operating frequency
/// 
/// Frequency can be set to any arbitrary value and directly controls speed of event loop.
/// Frequency shouldn't be too high since polling operations are currently blocking. No error
/// occurs if polling time exceeds frequency.
///
/// Refer to file notes about making this a mutable value
const FREQUENCY: std::time::Duration = std::time::Duration::from_secs(1);

/// █▓▒░ Load settings and setup `Group`.
///
/// # Args
/// name - Name to be converted to string
///
/// # Returns
/// Simgle initialized Group
fn init(name: &str) -> Group {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    println!("Initialized settings");

    let group = Group::new(name.clone(), Some(settings));
    println!("Initialized poll group: \"{}\"", name);
    group
}

/// █▓▒░ Setup and add devices to given `Group`.
///
/// Initial formatting for basic devices is demonstrated.
fn setup_poller(poller: &mut Group) {
    let config = vec![
        (
            "test name",
            0,
            IOKind::PH,
            IODirection::Input,
            IOCommand::Input(|| RawValue::Float(1.2)),
        ),
        (
            "second sensor",
            1,
            IOKind::Flow,
            IODirection::Input,
            IOCommand::Input(|| RawValue::Float(0.5)),
        ),
    ];
    poller.add_devices(&config).unwrap();
}

/// █▓▒░ Add a single `ThresholdNotifier` to all device in `Group`.
///
/// This demonstrates the initialization of `ThresholdNotifier` subscribers and shows how
/// subscribers are added to `Group` via `::.
fn build_subscribers(poller: &mut Group) {
    println!("\n█▓▒░ Building subscribers ...");

    let evaluator = EvaluationFunction::Threshold(
        |value, threshold| 
        threshold - value
    );

    for (id, input) in poller.inputs.iter() {
        println!("\n- Initializing builder ...");

        let mut builder = ActionBuilder::new(input.clone()).unwrap();

        println!("- Initializing subscriber ...");

        let name = format!("Subscriber for Input:{}", id);
        let threshold = RawValue::Float(1.0);
        let trigger = Comparison::GT;
        builder.add_threshold(&name, threshold, trigger, evaluator.clone(), None);
    }

    println!("\n... Finished Initializing subscribers\n");
}

/// █▓▒░ Handle polling of all devices in `Group`
fn poll(poller: &mut Group) -> Result<(), ErrorType> {
    match poller.poll() {
        Ok(_) => match poller.save(&None) {
            Ok(_) => println!("\n"),
            Err(t) => {
                return Err(t);
            },
        },
        _ => (),
    };
    Ok(())
}


fn main() {
    let mut poller = init("main");

    setup_poller(&mut poller);
    build_subscribers(&mut poller);

    println!("█▓▒░ Beginning polling ░▒▓█\n");

    loop {

        poll(&mut poller)
            .expect("Error occurred during polling");

        std::thread::sleep(FREQUENCY);

    }
}
