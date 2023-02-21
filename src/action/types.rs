//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::action::CommandType;
use crate::io::IOType;

// Command Factories
pub type BaseCommandFactory = fn(IOType, IOType) -> CommandType;