use crate::action::{Command, IOCommand};
use crate::errors::ErrorType;
use crate::helpers::Def;
use crate::io::{Datum, IOEvent};
use crate::storage::{Chronicle, Log};
use chrono::{DateTime, Utc};
use std::ops::Not;
use std::sync::{Arc, Mutex, Weak};

/// A [`Command`] that should be executed at a scheduled time *outside* of the normal event loop.
///
/// A weak reference to originating log is maintained so that logging of events is automatically
/// handled.
///
/// A [`Routine`] s intended for output events should be generated by [`IOEvent`]s returned from
/// incoming data. These events are usually time bound such as toggling a device to only keep it
/// active (or inactive) for a short period of time.
///
/// # Example
///
/// The primary use case is turning off a pump or other output after a predetermined period of time.
/// The normal event loop will execute the first action, but to avoid blocking the thread, a
/// [`Routine`] should be scheduled.
pub struct Routine {
    /// Scheduled time to execute function
    timestamp: DateTime<Utc>,

    /// Value to pass to `IOCommand`
    value: Datum,

    /// Weak reference to log for originating device
    log: Option<Weak<Mutex<Log>>>,

    /// Low-level command to execute
    command: IOCommand,
}

impl Routine {
    /// Constructor for [`Routine`]
    ///
    /// # Parameters
    ///
    /// - `timestamp`: Scheduled time of execution
    /// - `value`: Value to pass to command
    /// - `log`: Strong reference to [`Log`] which is internally
    ///   downgraded.
    /// - `command`: Low-level output command
    ///
    /// # Returns
    ///
    /// Initialized instance with scheduled time and downgraded reference
    /// to [`Log`]
    pub fn new<L>(timestamp: DateTime<Utc>, value: Datum, log: L, command: IOCommand) -> Self
    where
        L: Into<Option<Def<Log>>>,
    {
        // downgrade `Def` reference to `sync::Weak` reference
        let weak_log: Option<Weak<Mutex<Log>>>;
        if let Some(log) = log.into() {
            weak_log = Some(Arc::downgrade(&log.into()));
        } else {
            weak_log = None;
        }

        if command.is_output().not() {
            panic!("Command is not Output");
        }

        Self {
            timestamp,
            value,
            log: weak_log,
            command,
        }
    }

    /// Main polling function
    ///
    /// Acts as wrapper for [`Command::execute()`]. Checks scheduled time,
    /// then executes command. [`IOEvent`] is automatically added to device
    /// log.
    ///
    /// # Returns
    ///
    /// A `bool` that indicates:
    ///
    /// - `true`: if execution of [`IOCommand`] was successful indicating
    ///   instance should be dropped.
    /// - `false`: if [`IOCommand`] has not been executed. Instance should
    ///   not be dropped yet.
    pub fn attempt(&self) -> bool {
        let now = Utc::now();
        if now >= self.timestamp {
            let result = self.execute(self.value);
            match result {
                Ok(event) => {
                    let event = event.unwrap();
                    let _ = self.push_to_log(&event);
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

impl Command<IOEvent, ErrorType> for Routine {
    fn execute<V>(&self, value: V) -> Result<Option<IOEvent>, ErrorType>
    where
        V: Into<Option<Datum>>,
    {
        let value = value.into();
        match self.command.execute(value) {
            Ok(_) => {
                let event = IOEvent::with_timestamp(self.timestamp, value.unwrap());
                Ok(Some(event))
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl Chronicle for Routine {
    fn log(&self) -> Option<Def<Log>> {
        if let Some(weak_log) = self.log.clone() {
            if let Some(weak_ref) = weak_log.upgrade() {
                return Some(Def::from(weak_ref));
            }
        }
        None
    }
}

#[cfg(test)]
mod functionality_tests {
    use crate::action::{IOCommand, Routine};
    use crate::helpers::Def;
    use crate::io::{Datum, DeviceMetadata};
    use crate::storage::Log;
    use chrono::{Duration, Utc};

    const REGISTER_DEFAULT: Datum = Datum::Binary(Some(false));
    static mut REGISTER: Datum = REGISTER_DEFAULT;

    unsafe fn reset_register() {
        REGISTER = REGISTER_DEFAULT;
    }

    unsafe fn set_register(val: Datum) {
        REGISTER = val;
    }

    #[test]
    fn test_attempt() {
        unsafe {
            reset_register();
        }
        let metadata = DeviceMetadata::default();

        let log = Def::new(Log::with_metadata(&metadata));

        let command = IOCommand::Output(move |val| unsafe {
            set_register(val);
            Ok(())
        });

        let timestamp = Utc::now() + Duration::microseconds(10);
        let value = Datum::binary(true);
        let routine = Routine::new(timestamp, value, log.clone(), command);

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

    use crate::{
        action::{IOCommand, Routine},
        helpers::Def,
        io::{Datum, DeviceMetadata},
        storage::Log,
    };
    #[test]
    fn test_constructor_w_none() {
        let timestamp = Utc::now();
        let value = Datum::binary(true);
        let command = IOCommand::Output(|_| Ok(()));

        let routine = Routine::new(timestamp, value, None, command);

        assert!(routine.attempt());
    }

    #[test]
    fn test_constructor_w_log() {
        let metadata = DeviceMetadata::default();

        let log = Def::new(Log::with_metadata(&metadata));

        let timestamp = Utc::now();
        let value = Datum::binary(true);
        let command = IOCommand::Output(|_| Ok(()));

        let routine = Routine::new(timestamp, value, log.clone(), command);
        assert!(routine.attempt());
    }

    #[test]
    #[should_panic]
    fn validate_command() {
        let timestamp = Utc::now();
        let value = Datum::binary(true);
        let command = IOCommand::Input(|| Datum::default());

        let routine = Routine::new(timestamp, value, None, command);
        assert!(routine.attempt());
    }
}
