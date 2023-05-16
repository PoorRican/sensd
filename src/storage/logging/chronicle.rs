use crate::helpers::Def;
use crate::io::IOEvent;
use crate::storage::Log;

/// Interface for interacting with [`Log`] behind a [`Def`] guard.
pub trait Chronicle {
    /// Property to return reference to field
    ///
    /// # Panics
    ///
    /// If stored reference to `Log` is [`std::sync::Weak`] and cannot be upgraded.
    ///
    /// # Returns
    ///
    /// An `Option` with `Some` a [`Log`] is assigned, otherwise `None`.
    fn log(&self) -> Option<Def<Log>>;

    /// Appends [`IOEvent`] to collection
    ///
    /// If there is no associated `Log`, method silently fails.
    ///
    /// # Panics
    ///
    /// - If underlying [`Def<Log>`] reference is poisoned and cannot be locked.
    /// - When an error occurs during [`Log::push()`]
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
    /// Underlying reference is not checked or validated.
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
