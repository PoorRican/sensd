use crate::io::{IOEvent, IOType};
use crate::errors::ErrorType;


pub type CommandType = Box<dyn Command>;

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self, value: Option<IOType>) -> Result<Option<IOEvent>, ErrorType>;
}
