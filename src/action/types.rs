//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::action::BoxedCommand;
use crate::io::{IODirection, IOEvent, IOType};

// Command Factories
#[derive(Clone, Copy)]
pub enum IOCommand {
    Input(fn() -> IOType),
    Output(fn(IOType) -> Result<(), ()>),
}

impl IOCommand {
    pub fn direction(&self) -> IODirection {
        match self {
            IOCommand::Input(_) => IODirection::Input,
            IOCommand::Output(_) => IODirection::Output,
        }
    }
}

pub type ThresholdFactory = fn(IOType, IOType) -> BoxedCommand<IOEvent>;
