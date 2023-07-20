use crate::io::Datum;

pub type BoxedCommand<T, E> = Box<dyn Command<T, E>>;

/// Interface for executing a single atomic operation
pub trait Command<T, E> {
    /// Execute arbitrary command
    ///
    /// # Parameters
    /// - `value`: Arbitrary value to be passed to command.
    ///            This is used by [`crate::action::IOCommand::Output`]. A warning is printed to
    ///            stderr if `value` is not `None` when called from [`crate::action::IOCommand::Input`].
    ///
    /// # Returns
    /// - `Ok(T)`: returned when execution completes without error.
    /// - `Err(ErrorType)`: [`ErrorType`] is returned when an error occurs during operation.
    fn execute<V>(&self, value: V) -> Result<Option<T>, E>
    where
        V: Into<Option<Datum>>;
}
