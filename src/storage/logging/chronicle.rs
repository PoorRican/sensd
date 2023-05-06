use crate::helpers::Def;
use crate::io::IOEvent;
use crate::storage::Log;

/// Transparently enables a reference to `Log` to be shared.
pub trait Chronicle {
    /// Property to return reference to field
    ///
    /// Upgrading of `Weak` reference should occur here
    fn log(&self) -> Option<Def<Log>>;

    fn add_to_log(&self, event: IOEvent) {
        if let Some(log) = self.log() {
            log.try_lock()
                .unwrap()
                .push(event.timestamp, event)
                .expect("Unknown error when adding event to log");
        }
    }

    fn has_log(&self) -> bool {
        match self.log() {
            Some(_) => true,
            None => false,
        }
    }
}
