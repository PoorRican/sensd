//! An example of demonstrating a bang-bang (on-off) controller to maintain temperature.
//!
//! This is intended to be a rough implementation for the RPi
//! and to observe the PID controller in action.
extern crate chrono;
extern crate sensd;
extern crate serde;

use std::error::Error;
use sensd::action::{Action, actions, IOCommand, Trigger};
use sensd::errors::ErrorType;
use sensd::io::{Device, IdType, Input, Output, Datum};
use sensd::storage::{Group, Persistent};

use std::ops::DerefMut;
use std::thread;
#[cfg(feature = "rppal")]
use rppal::gpio::{Gpio, Level};
use sensd::name::Name;

const HEATER_PIN: u8 = 23;
const SENSOR_PIN: u8 = 24;

/// Example setpoint of 92.0F
const SETPOINT: f32 = 92.0;

/// █▓▒░ Event Loop Operating frequency
///
/// Frequency can be set to any arbitrary value and directly controls speed of event loop.
/// Frequency shouldn't be too high since polling operations are currently blocking. No error
/// occurs if polling time exceeds frequency.
///
/// Refer to file notes about making this a mutable value
const FREQUENCY: std::time::Duration = std::time::Duration::from_secs(5);


#[cfg(not(feature = "rppal"))]
fn main() {
    println!("This example needs to be run on an Raspberry Pi")
}

#[cfg(feature = "rppal")]
fn main() -> Result<(), Box<dyn Error>>{

    // initialize HW pins
    let mut heater = Gpio::new()?
        .get(HEATER_PIN)?
        .into_output();
    let sensor = Gpio::new()?
        .get(SENSOR_PIN)?
        .into_input();

    let mut poller = Group::new("main");


    // build output
    poller.push_output_then(
        Output::new(OUTPUT_ID)
            .set_command(IOCommand::Output(|val| {
                if let Datum::Binary(inner) = val {
                    if let Some(value) = inner {
                        let output = match value {
                            true => Level::High,
                            false => Level::Low
                        };
                        heater.write(output);
                        return Ok(())
                    }
                }
                panic!("Incorrect value passed to output command")
            }))
            .init_log()
    );

    // build input + PID controller
    poller.push_input_then(
        Input::new(0)
            .set_command(IOCommand::Input(|| Datum::float(sensor.read())))
            .init_log()
            .init_publisher()
            .set_name("heater");
    let mut publisher = input.publisher_mut().unwrap();
    publisher.subscribe(
        actions::PID::new("heater PID", SETPOINT, 1000.0).into_boxed()
    );
    drop(publisher);
    poller.push_input(input);

    loop {
        poller.poll()
            .and_then(poller.save().expect("Could not save"))?;
        thread::sleep(FREQUENCY);
    }
}