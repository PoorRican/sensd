use crate::io::{IOEvent, IOType};
use crate::errors::Result;


pub type CommandType = Box<dyn Command>;

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self, value: Option<IOType>) -> Option<Result<IOEvent>>;
}
