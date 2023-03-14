use crate::action::{Command, GPIOCommand};
use crate::errors::ErrorType;
use crate::helpers::Deferred;
use crate::io::{DeviceMetadata, IOEvent, RawValue};
use crate::storage::{HasLog, Log};
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex, Weak};

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
    ///
    /// A copy is used to avoid locking issues since scheduled commands might be time critical.
    metadata: DeviceMetadata,

    /// Value to pass to `GPIOCommand`
    value: RawValue,

    /// Weak reference to log for originating device
    log: Weak<Mutex<Log>>,

    command: GPIOCommand,
}

impl Routine {
    pub fn new(
        timestamp: DateTime<Utc>,
        metadata: DeviceMetadata,
        value: RawValue,
        log: Deferred<Log>,
        command: GPIOCommand,
    ) -> Self {
        let log = Arc::downgrade(&log);
        Self {
            timestamp,
            metadata,
            value,
            log,
            command,
        }
    }
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
                    let _ = self.add_to_log(event);
                    return true;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        };

        // return false by default
        false
    }
}

impl Command<IOEvent> for Routine {
    fn execute(&self, value: Option<RawValue>) -> Result<Option<IOEvent>, ErrorType> {
        match self.command.execute(value) {
            Ok(_) => {
                let event = IOEvent::generate(&self.metadata, self.timestamp, value.unwrap());
                Ok(Some(event))
            }
            Err(e) => Err(e),
        }
    }
}

impl HasLog for Routine {
    fn log(&self) -> Option<Deferred<Log>> {
        Some(self.log.upgrade().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::action::{GPIOCommand, IOCommand, Routine};
    use crate::helpers::Deferrable;
    use crate::io::{DeviceMetadata, RawValue};
    use crate::storage::{Log, MappedCollection};
    use chrono::{Duration, Utc};

    const REGISTER_DEFAULT: RawValue = RawValue::Binary(false);
    static mut REGISTER: RawValue = REGISTER_DEFAULT;

    unsafe fn reset_register() {
        REGISTER = REGISTER_DEFAULT;
    }

    unsafe fn set_register(val: RawValue) {
        REGISTER = val;
    }

    #[test]
    fn test_attempt() {
        unsafe {
            reset_register();
        }
        let metadata = DeviceMetadata::default();

        let log = Log::new(metadata.id, None).deferred();

        let func = IOCommand::Output(
            move |val| unsafe {
            set_register(val);
            Ok(())
        });
        let command = GPIOCommand::new(func, None);

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = RawValue::Binary(true);
        let routine = Routine::new(timestamp, metadata, value, log.clone(), command);

        unsafe {
            assert_ne!(REGISTER, value);
        }

        while Utc::now() < timestamp {
            assert_eq!(false, routine.attempt());
        }

        assert!(routine.attempt());
        unsafe {
            assert_eq!(REGISTER, value);
        }
        assert_eq!(log.try_lock().unwrap().length(), 1);
    }
}
