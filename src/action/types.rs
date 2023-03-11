//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::io::{IODirection, IOType};

/// Command Factories
///
/// The internal functions should accept arbitrary HAL code.
#[derive(Clone)]
pub enum IOCommand {
    Input(fn() -> IOType),
    Output(fn(IOType) -> Result<(), ()>),
}

impl IOCommand {
    /// Return direction of `IOCommand` Type
    ///
    /// This is used to check that input devices accept input commands, vice versa.
    pub fn direction(&self) -> IODirection {
        match self {
            IOCommand::Input(_) => IODirection::Input,
            IOCommand::Output(_) => IODirection::Output,
        }
    }
}

/// Generate value to pass to `IOCommand`
///
/// Function arguments should be subscriber specific data and data returned by input device.
#[derive(Clone)]
pub enum EvaluationFunction {
    // Calculate value to write based on threshold and input value
    Threshold(fn(value: IOType, threshold: IOType) -> IOType)
}
