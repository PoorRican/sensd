//! A basic example of sensd implementation.
//!
//! # Description
//!
//! This example displays the use of the "ThresholdAction" subscriber. Two input devices are
//! initialized using the `DeviceLog` builder - both devices return a static float value.
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
//! static frequency `FREQUENCY`. There might be a use case where frequency needs to be modulated,
//! such as during a control cycle for more accurate control. Reduced polling time might be useful
//! in embedded scenarios requiring both power conservation and accurate control.
extern crate chrono;
extern crate sensd;
extern crate serde;

use sensd::action::{Action, actions, IOCommand, Trigger};
use sensd::errors::ErrorType;
use sensd::io::{IOKind, Datum, Input, Device};
use sensd::storage::{Group, Persistent};

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
/// Single initialized Group
fn init(name: &str) -> Group {
    let group = Group::new(name.clone());
    println!("Initialized poll group: \"{}\"", name);
    group
}

/// █▓▒░ Handle polling of all devices in `Group`
fn poll(poller: &mut Group) -> Result<(), ErrorType> {
    match poller.poll() {
        Ok(_) => match poller.save() {
            Ok(_) => println!("\n"),
            Err(t) => {
                return Err(t);
            }
        },
        _ => (),
    };
    Ok(())
}

fn main() {
    let mut poller = init("main");

    // setup ph sensor
    {
        let name = "test name";
        let id = 0;
        let kind = IOKind::PH;
        let command = IOCommand::Input(|| Datum::float(1.2));

        // build input device
        let mut input =
            Input::new(
                name,
                id,
                Some(kind),
            ).set_command(
                command
            ).init_log(
            ).init_publisher();

        // setup publisher/action
        input.publisher_mut().as_mut().unwrap()
            .subscribe(
                actions::Threshold::new(
                    format!("Subscriber for Input:{}", id),
                    Datum::float(1.0),
                    Trigger::GT,
                ).into_boxed()
            );

        poller.push_input(input);
    }
    // setup flow sensor
    {
        let name = "second sensor";
        let id = 1;
        let kind = IOKind::PH;
        let command = IOCommand::Input(|| Datum::float(1.2));

        // build input device
        let mut input = Input::new(
            name,
            id,
            kind,
        )
            .set_command(command)
            .init_log()
            .init_publisher();

        // setup publisher/action
        input.publisher_mut().as_mut().unwrap()
            .subscribe(
                actions::Threshold::new(
                    format!("Subscriber for Input:{}", id),
                    Datum::float(1.0),
                    Trigger::GT,
                ).into_boxed()
            );

        poller.push_input(input);
    }

    println!("█▓▒░ Beginning polling ░▒▓█\n");

    loop {
        poll(&mut poller).expect("Error occurred during polling");

        std::thread::sleep(FREQUENCY);
    }
}
