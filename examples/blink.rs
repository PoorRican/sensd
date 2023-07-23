//! The classic "hello world" for embedded programming: the blink program...
//!
//! # Description
//! No control logic is implemented here. An output device is initialized and used here. Therefore,
//! this example may be used as the basis for forming schedules such as irrigation or feeding
//! routines.
extern crate chrono;
extern crate sensd;
extern crate serde;

use sensd::action::IOCommand;
use sensd::io::{IdType, Datum, Input, Device};
use sensd::storage::{Group, Persistent};
use std::ops::{DerefMut, Neg};
use sensd::name::Name;

/// █▓▒░ Event Loop Operating frequency
///
/// Frequency can be set to any arbitrary value and directly controls speed of event loop.
/// Frequency shouldn't be too high since polling operations are currently blocking. No error
/// occurs if polling time exceeds frequency.
const FREQUENCY: std::time::Duration = std::time::Duration::from_secs(1);

/// Hardcoded ID for output device
const OUTPUT_ID: IdType = 0;

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

/// █▓▒░ Build and add devices to `Group`.
///
/// Use of `Group::add_devices()` is demonstrated.
fn setup_devices(poller: &mut Group) {
    poller.push_input_then(
        Input::new(
            OUTPUT_ID,
        ).set_command(
            IOCommand::Output(|val| Ok(println!("\n{}\n", val)))
        ).set_name("Mock Output")
    );
}

fn main() {
    let mut poller = init("main");
    setup_devices(&mut poller);

    let wrapped_device = poller
        .outputs
        .get(&OUTPUT_ID)
        .expect("Unknown error when retrieving device")
        .clone();

    println!("█▓▒░ Beginning loop ░▒▓█\n");

    let mut value = Datum::binary(false);

    loop {
        {
            let mut binding = wrapped_device.try_lock().unwrap();
            binding.deref_mut()
                .write(value)
                .expect("Error while calling `::write()` on output device");
        }

        poller.save().expect("Error while saving");

        value = value.neg();    // alternate output value

        std::thread::sleep(FREQUENCY);
    }
}
