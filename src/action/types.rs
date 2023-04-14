//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::io::{IODirection, RawValue};

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

/// Container for data processing functions.
///
/// Variants are function types that at minimum accept input data, and return a `RawValue` to be
/// passed to `IOCommand`.
///
// TODO: rename to "Evaluator"
// TODO: Return type for threshold should be `Option`
#[derive(Clone)]
pub enum EvaluationFunction {
    /// Calculate value to write based on threshold parameter and input value
    Threshold(fn(value: RawValue, threshold: RawValue) -> RawValue)
}
