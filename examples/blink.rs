//! The classic "hello world" for embedded programming: the blink program...
//!
//! # Description 
//! No control logic is implemented here. An output device is initialized and used here. Therefore,
//! this example may be used as the basis for forming schedules such as irrigation or feeding
//! routines.
extern crate chrono;
extern crate sensd;
extern crate serde;

use std::sync::Arc;

use sensd::action::IOCommand;
use sensd::io::{IODirection, IOKind, RawValue, IdType, DeviceType};
use sensd::settings::Settings;
use sensd::storage::{Group, Persistent};
use std::ops::DerefMut;

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
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    println!("Initialized settings");

    let group = Group::new(name.clone(), Some(settings));
    println!("Initialized poll group: \"{}\"", name);
    group
}

/// █▓▒░ Build and add devices to `Group`.
///
/// Use of `Group::add_devices()` is demonstrated.
fn setup_devices(poller: &mut Group) {
    let config = vec![
        (
            "Mock Output",
            OUTPUT_ID,
            IOKind::Light,
            IODirection::Output,
            IOCommand::Output(|val| Ok(println!("\n{}\n", val))),
        ),
    ];
    poller.add_devices(&config).unwrap();
}

/// Alternate boolean value to pass to output.
///
/// Boolean value is modified 
fn alternate_value(value: &mut RawValue) {
    if let RawValue::Binary(inner) = value {
        *value = match inner {
            true => RawValue::Binary(false),
            false => RawValue::Binary(true),
        };
    } else {
        panic!("Variant is not `RawValue::Binary`");
    }
}

fn main() {
    let mut poller = init("main");
    setup_devices(&mut poller);

    let wrapped_device = poller.outputs.get(&OUTPUT_ID)
        .expect("Unknown error when retrieving device").clone();

    println!("█▓▒░ Beginning loop ░▒▓█\n");

    let mut value = RawValue::Binary(false);

    loop {

        {
            let mut binding = wrapped_device.try_lock().unwrap();
            let output_device = binding.deref_mut();

            if let DeviceType::Output(output) = output_device {

                output.write(value)
                    .expect("Error while calling `::write()` on output device");

            };
        }

        poller.save(&None).expect("Error while saving");

        alternate_value(&mut value);            // alternate output value

        std::thread::sleep(FREQUENCY);

    }
}
