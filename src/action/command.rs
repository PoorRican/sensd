use crate::errors::ErrorType;
use crate::io::IOType;

pub type CommandType<T> = Box<dyn Command<T>>;

/// Abstraction for single atomic output operation
pub trait Command<T> {
    fn execute(&self, value: Option<IOType>) -> Result<Option<T>, ErrorType>;
}
