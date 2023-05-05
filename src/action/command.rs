use crate::errors::ErrorType;
use crate::io::RawValue;

pub type BoxedCommand<T> = Box<dyn Command<T>>;

/// Interface for executing a single atomic operation
pub trait Command<T> {
    /// Execute arbitrary command
    ///
    /// # Parameters
    /// - `value`: Arbitrary value to be passed to command.
    ///            This is used by [`IOCommand::Output`]. A warning is printed to
    ///            stderr if `value` is not `None` when called from [`IOCommand::Input`].
    ///
    /// # Returns
    /// - `Ok(T)`: returned when execution completes without error.
    /// - `Err(ErrorType)`: [`ErrorType`] is returned when an error occurs during operation.
    fn execute(&self, value: Option<RawValue>) -> Result<Option<T>, ErrorType>;
}
