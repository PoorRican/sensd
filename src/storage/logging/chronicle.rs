use crate::helpers::Def;
use crate::io::IOEvent;
use crate::storage::Log;

/// Interface for an object that uses with [`Def<Log>`]
pub trait Chronicle {
    /// Getter for reference to log
    ///
    /// # Panics
    ///
    /// If stored reference to `Log` is [`std::sync::Weak`] and cannot be upgraded.
    ///
    /// # Returns
    ///
    /// An `Option` that contains:
    ///
    /// - `Some` if a [`Log`] is assigned
    /// - `None` if no [`Log`]
    fn log(&self) -> Option<Def<Log>>;

    /// Appends [`IOEvent`] to collection
    ///
    /// Silently fails if there is no associated [`Log`].
    ///
    /// # Parameters
    ///
    /// - `event`: [`IOEvent`] to add to [`EventCollection`]
    ///
    /// # Panics
    ///
    /// - If underlying [`Def<Log>`] reference is poisoned and cannot be locked.
    /// - When an error occurs during [`Log::push()`]
    ///
    /// # See Also
    ///
    /// - [`Log::push()`] for how [`IOEvent`] is added to [`EventCollection`]
    fn push_to_log(&self, event: &IOEvent) {
        if let Some(log) = self.log() {
            log.try_lock()
                .expect("Could not lock `Log`")
                .push(event.clone())
                .expect("Error when adding event to log");
        }
    }

    /// Simple check to see if a [`Log`] is assigned or not
    ///
    /// Underlying reference is not checked or validated. Therefore, this method
    /// does not fail if reference is poisoned.
    ///
    /// # Returns
    ///
    /// - `true`: when log is assigned
    /// - `false`: when log is `None`
    fn has_log(&self) -> bool {
        match self.log() {
            Some(_) => true,
            None => false,
        }
    }
}
