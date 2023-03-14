use crate::errors::ErrorType;
use crate::io::RawValue;

pub type BoxedCommand<T> = Box<dyn Command<T>>;

/// Abstraction for single atomic output operation
pub trait Command<T> {
    /// Execute arbitrary command
    ///
    /// # Args
    /// value: Arbitrary value to be passed to command.
    ///        This is used by `IOCommand::Output`.
    ///
    /// # Returns
    /// Result containing optional value for `Ok` or `ErrorType`
    fn execute(&self, value: Option<RawValue>) -> Result<Option<T>, ErrorType>;
}
