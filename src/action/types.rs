//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::io::{IODirection, RawValue};
use crate::errors::ErrorType;

use super::Command;

/// Implementation of Command design pattern for low-level I/O code
///
/// Should be used as an interface for HAL code and otherwise perform no other logic. Data
/// processing, or formatting of I/O values should be handled by `EvaluationFunction`.
#[derive(Clone)]
pub enum IOCommand {
    /// Low-level code to read HW input
    Input(fn() -> RawValue),
    /// Low-level code to write to HW output
    ///
    /// # Returns
    /// `Err` is returned if `RawValue` variant is incorrect. Otherwise, `Ok` is returned by
    /// default.
    Output(fn(RawValue) -> Result<(), ()>),
}

impl IOCommand {
    /// Get direction of `IOCommand` instance.
    ///
    /// Used to verify device type aligns with function intention: input with input, vice versa.
    pub fn direction(&self) -> IODirection {
        match self {
            IOCommand::Input(_) => IODirection::Input,
            IOCommand::Output(_) => IODirection::Output,
        }
    }
}

impl Command<RawValue> for IOCommand {
    /// Execute internally stored function.
    ///
    /// In summary, input command returns a value, output command accepts a value.
    ///
    /// # Args
    /// value: Arbitrary value to pass to output. If passed to an input, an error is printed, but no panic occurs.
    ///
    /// # Returns
    /// If internal function is `IOCommand::Input`, then the value that is read from device is returned.
    /// Otherwise, if `IOCommand::Output`, then `None` is returned.
    fn execute(&self, value: Option<RawValue>) -> Result<Option<RawValue>, ErrorType> {
        match self {
            Self::Input(inner) => {
                // throw warning for unused value
                if let Some(_) = value {
                    unused_value()
                }

                let read_value = inner();

                Ok(Some(read_value))
            }
            Self::Output(inner) => {
                let unwrapped_value = value.expect("No value was passed to write...");
                let _ = inner(unwrapped_value); // TODO: handle bad result

                Ok(None)
            }
        }
    }
}


/// Print a warning on console stderr
fn unused_value() {
    const MSG: &str = "Unused value passed when reading input...";
    eprintln!("{}", MSG);
}
