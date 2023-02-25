use std::ops::Deref;
use crate::action::{Command, CommandType, IOCommand};
use crate::io::{DeferredDevice, Device, DeviceType, IOEvent, IOType, DeviceTraits};
use crate::errors::{Error, ErrorKind, ErrorType};

pub struct GPIOCommand {
    func: IOCommand,
}

impl GPIOCommand {
    pub fn new(func: IOCommand, device: DeferredDevice) -> Self {
        check_alignment(&func, device.clone());

        Self { func }
    }

    /// Execute internally stored function.
    ///
    /// # Returns
    /// If internal function is `IOCommand::Input`, then the value that is read from device is returned.
    /// Otherwise, if `IOCommand::Output`, then `None` is returned.
    pub fn execute(&self, value: Option<IOType>) -> Result<Option<IOType>, ErrorType> {
        match self.func {
            IOCommand::Input(inner) => {
                // throw warning for unused value
                if let Some(_) = value { unused_value() }

                let read_value = inner();

                Ok(Some(read_value))

            },
            IOCommand::Output(inner) => {
                let unwrapped_value = value.expect("No value was passed to write...");
                let _ = inner(unwrapped_value);       // TODO: handle bad result

                Ok(None)
            },
        }
    }
}

/// Panic if command and device are not aligned
fn check_alignment(command: &IOCommand, device: DeferredDevice) {
    assert_eq!(command.direction(), device.direction())
}

/// Generate an error for when command type does not match device type
fn misconfigured_error() -> Box<dyn std::error::Error> {
    Error::new(ErrorKind::CommandError, "Misconfigured device! Device and command type do not match.")
}

/// Print a warning on console stderr
fn unused_value() {
    const MSG: &str = "Unused value passed when reading input...";
    eprintln!("{}", MSG);
}