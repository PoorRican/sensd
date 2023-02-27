use std::sync::{Mutex, Weak};
use chrono::{DateTime, Utc};
use crate::action::{GPIOCommand, Command};
use crate::errors::ErrorType;
use crate::helpers::Deferred;
use crate::io::{IOEvent, IOType, DeviceMetadata};
use crate::storage::{MappedCollection, OwnedLog};

/// A `Command` that should be executed at a scheduled time *outside* of the normal event loop.
///
/// Typically these should exclusively be `Output` events, such as completing a time bound operation.
///
/// # Example
/// The primary use case is turning off a pump or other output after a predetermined period of time.
/// The normal event loop will execute the first action, but to avoid blocking the thread, a `Routine`
/// should be scheduled.
pub struct Routine {
    /// Scheduled time to execute function
    timestamp: DateTime<Utc>,

    /// Copy of owning device metadata
    /// A copy is used to avoid locking issues since scheduled commands might be time critical.
    metadata: DeviceMetadata,

    /// Value to pass to `GPIOCommand`
    value: IOType,

    /// Weak reference to log for originating device
    log: Weak<Mutex<OwnedLog>>,

    command: GPIOCommand,
}

impl Routine {
    /// Main polling function
    ///
    /// Checks scheduled time, then executes command. `IOEvent` is automatically added to device log.
    ///
    /// # Returns
    /// bool based on if execution was successful or not. This value should be used to drop `Routine` from
    /// external store.
    pub fn attempt(&self) -> bool {
        let now = Utc::now();
        if now >= self.timestamp {
            let result = self.execute(Some(self.value));
            match result {
                Ok(event) => {
                    let event = event.unwrap();
                    self.add_to_log(event);
                    return true
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        };

        // return false by default
        false
    }

    fn add_to_log(&self, event: IOEvent) {
        let log: Deferred<OwnedLog> = self.log.upgrade().unwrap();
        // TODO: This will panic in a multi-threaded context, should `IOEvents` be sent to a queue?
        //      Event will then be added to device log *by* device during polling
        log.try_lock().unwrap().push(event.timestamp, event)
            .expect("Unknown error occurred when attempting to add to device log.");
    }
}

impl Command<IOEvent> for Routine {
    fn execute(&self, value: Option<IOType>) -> Result<Option<IOEvent>, ErrorType> {
        let event = IOEvent::generate(&self.metadata, self.timestamp, value.unwrap());
        Ok(Some(event))
    }
}