use crate::action::{Command, IOCommand};
use crate::errors::ErrorType;
use crate::helpers::Def;
use crate::io::{DeviceMetadata, IOEvent, RawValue};
use crate::storage::{Chronicle, Log};
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex, Weak};

/// A `Command` that should be executed at a scheduled time *outside* of the normal event loop.
///
/// A weak reference to originating log is maintained so that logging of events is automatically
/// handled.
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
    /// A copy of metadata avoids possible locking issues from deferred [`DeviceMetadata`] types.
    /// Avoidance of locking issues is crucial to since execution of [`Routine`] is assumed to be
    /// critical and should be executed in real-time.
    // TODO: create as optional once issue #96 has been implemented
    metadata: DeviceMetadata,

    /// Value to pass to `IOCommand`
    value: RawValue,

    /// Weak reference to log for originating device
    log: Option<Weak<Mutex<Log>>>,

    command: IOCommand,
}

impl Routine {
    pub fn new<M, L>(
        timestamp: DateTime<Utc>,
        metadata: M,
        value: RawValue,
        log: L,
        command: IOCommand,
    ) -> Self where 
        M: Into<Option<DeviceMetadata>>,
        L: Into<Option<Def<Log>>>,
    {
        // downgrade `Def` reference to `sync::Weak` reference
        let weak_log: Option<Weak<Mutex<Log>>>;
        if let Some(log) = log.into() {
            weak_log = Some(Arc::downgrade(&log.into()));
        } else {
            weak_log = None;
        }

        let metadata: DeviceMetadata = metadata.into().unwrap_or_default();

        Self {
            timestamp,
            metadata,
            value,
            log: weak_log,
            command,
        }
    }

    /// Main polling function 
    ///
    /// Acts as wrapper for [`Command::execute()`]. Checks scheduled time, then executes command.
    /// `IOEvent` is automatically added to device log.
    ///
    /// # Returns
    /// The returned value should be used for dropping [`Routine`] from external collection
    /// `true`: if execution of [`IOCommand`] was successful
    /// `false`: if [`IOCommand`] has not been executed
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
                let event = IOEvent::new(&self.metadata, self.timestamp, value.unwrap());
                Ok(Some(event))
            }
            Err(e) => Err(e),
        }
    }
}

impl Chronicle for Routine {
    fn log(&self) -> Option<Def<Log>> {
        if let Some(weak_log) = self.log.clone() {
            if let Some(weak_ref) = weak_log.upgrade() {
                return Some(Def::from(weak_ref))
            }
        } 
        None
    }
}

#[cfg(test)]
mod functionality_tests {
    use crate::action::{IOCommand, Routine};
    use crate::helpers::Def;
    use crate::io::{DeviceMetadata, RawValue};
    use crate::storage::Log;
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

        let log = Def::new(Log::new(metadata.id, None));

        let command = IOCommand::Output(
            move |val| unsafe {
            set_register(val);
            Ok(())
        });

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
        assert_eq!(log.try_lock().unwrap().iter().count(), 1);
    }
}

#[cfg(test)]
mod meta_tests {
    use chrono::Utc;

    use crate::{io::{DeviceMetadata, RawValue}, action::{IOCommand, Routine}, storage::Log, helpers::Def};
    #[test]
    fn test_constructor_w_none() {
        let timestamp = Utc::now();
        let value = RawValue::Binary(true);
        let command = IOCommand::Output(|_| { Ok(()) });

        let routine = Routine::new(timestamp, None, value, None, command);

        assert!(routine.attempt());
    }

    #[test]
    fn test_constructor_w_device() {
        let metadata = DeviceMetadata::default();

        let timestamp = Utc::now();
        let value = RawValue::Binary(true);
        let command = IOCommand::Output(|_| { Ok(()) });

        let routine = Routine::new(timestamp, metadata, value, None, command);

        assert!(routine.attempt());
    }

    #[test]
    fn test_constructor_w_log() {
        let log = Def::new(Log::default());

        let timestamp = Utc::now();
        let value = RawValue::Binary(true);
        let command = IOCommand::Output(|_| { Ok(()) });


        let routine = Routine::new(timestamp, None, value, log.clone(), command);
        assert!(routine.attempt());
    }

    #[test]
    fn test_constructor_w_both() {
        let metadata = DeviceMetadata::default();

        let log = Def::new(Log::default());

        let timestamp = Utc::now();
        let value = RawValue::Binary(true);
        let command = IOCommand::Output(|_| { Ok(()) });


        let routine = Routine::new(timestamp, metadata, value, log.clone(), command);
        assert!(routine.attempt());
    }
}
