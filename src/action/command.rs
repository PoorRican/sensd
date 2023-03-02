use crate::io::IOType;
use crate::errors::ErrorType;


pub type CommandType<T> = Box<dyn Command<T>>;

/// Abstraction for single atomic output operation
pub trait Command<T> {
    fn execute(&self, value: Option<IOType>) -> Result<Option<T>, ErrorType>;
}
