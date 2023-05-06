//! An example of demonstrating a bang-bang (on-off) controller to maintain temperature.
//!
//! This example is non-functional and simulates HW GPIO via `println()`.
//!
//! # Operation
//!
//! So the whole point is to display the use of an Output device via pub/sub functionality. This is
//! accomplished by oscillating `EXTERNAL_VALUE`. When the external value falls below `THRESHOLD`,
//! a message is printed to stdout to simulate a HW action.
//!
//! # Note
//!
//! ## █▓▒░ Unsafe Code
//! In order to simulate an external device,
extern crate chrono;
extern crate sensd;
extern crate serde;

use sensd::action::{Comparison, IOCommand};
use sensd::errors::ErrorType;
use sensd::io::{DeferredDevice, DeviceType, IODirection, IOKind, IdType, RawValue};
use sensd::settings::Settings;
use sensd::storage::{Group, Persistent};
use std::ops::DerefMut;

use std::sync::Arc;

const INPUT_ID: IdType = 0;
const OUTPUT_ID: IdType = 1;

/// █▓▒░ Event Loop Operating frequency
///
/// Frequency can be set to any arbitrary value and directly controls speed of event loop.
/// Frequency shouldn't be too high since polling operations are currently blocking. No error
/// occurs if polling time exceeds frequency.
///
/// Refer to file notes about making this a mutable value
const FREQUENCY: std::time::Duration = std::time::Duration::from_secs(5);

const THRESHOLD: i8 = 10;
static mut EXTERNAL_VALUE: RawValue = RawValue::Int8(0);

/// █▓▒░ Load settings and setup `Group`.
///
/// # Args
/// name - Name to be converted to string
///
/// # Returns
/// Single initialized Group
fn init(name: &str) -> Group {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    println!("Initialized settings");

    let group = Group::new(name.clone(), Some(settings));
    println!("Initialized poll group: \"{}\"", name);
    group
}

/// █▓▒░ Add devices to `Group`
///
/// Initial formatting for when using the `Group::add_device()`  is demonstrated.
unsafe fn setup_poller(poller: &mut Group) {
    let config = vec![
        (
            "mock temp sensor",
            INPUT_ID,
            IOKind::Temperature,
            IODirection::Input,
            IOCommand::Input(|| EXTERNAL_VALUE),
        ),
        (
            "test mock cooling device",
            OUTPUT_ID,
            IOKind::Temperature,
            IODirection::Output,
            IOCommand::Output(|val| Ok(println!("\nSimulated HW Output: {}\n", val))),
        ),
    ];
    poller.add_devices(&config).unwrap();
}

/// █▓▒░ Add a single `ThresholdNotifier` to all device in `Group`.
fn build_actions(poller: &mut Group) {
    println!("\n█▓▒░ Building subscribers ...");

    let input: DeferredDevice = poller.inputs.get(&INPUT_ID).unwrap().clone();
    let output: DeferredDevice = poller.outputs.get(&OUTPUT_ID).unwrap().clone();

    if let DeviceType::Input(device) = input.try_lock().unwrap().deref_mut() {
        println!("- Initializing subscriber ...");

        let name = format!("Subscriber for Input:{}", INPUT_ID);
        let threshold = RawValue::Int8(THRESHOLD);
        let trigger = Comparison::LT;
        if let Some(publisher) = device.publisher_mut() {
            publisher.attach_threshold(&name, threshold, trigger, Some(output.clone()));
        }
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
            }
        },
        _ => (),
    };
    Ok(())
}

fn main() {
    let mut poller = init("main");
    unsafe { setup_poller(&mut poller) }

    build_actions(&mut poller);

    println!("█▓▒░ Beginning polling ░▒▓█\n");

    let range = 5..11;
    for value in range.clone().into_iter().chain(range.rev()).cycle() {
        unsafe {
            EXTERNAL_VALUE = RawValue::Int8(value);
        }

        poll(&mut poller).expect("Error occurred during polling");

        std::thread::sleep(FREQUENCY);
    }
}
