use crate::action::Routine;

#[allow(unused_imports)]
use crate::storage::Group;

/// Wrapper for a collection of scheduled [`Routine`] instances that handles real-time execution
#[derive(Default)]
pub struct SchedRoutineHandler(Vec<Routine>);

impl SchedRoutineHandler {
    /// Add a [`Routine`] to the back of internal collection
    pub fn push(&mut self, routine: Routine) {
        self.0.push(routine)
    }

    /// Attempt to execute scheduled routines.
    ///
    /// Even though [`Routine`] instances are usually scheduled during normal polling cycles by
    /// [`Group`], the assumption is that their scheduled execution time does not correllate with a
    /// polling interval. Therefore, `attempt()` should be called as often as possible,
    /// outside of normal polling cycle, and as often as possible to produce real-time response.
    ///
    /// Any routines executed by `Routine::attempt()` are cleared from the `scheduled` container.
    pub fn attempt(&mut self) {
        let mut executed = Vec::default();
        for (index, routine) in self.0.iter().enumerate() {
            if routine.attempt() {
                executed.push(index);
            }
        }
        // remove completed
        for index in executed {
            self.0.remove(index);
        }
    }

    /// Getter function for internal collection
    pub fn scheduled(&self) -> &[Routine] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};

    use crate::{
        action::{SchedRoutineHandler, IOCommand, Routine},
        io::{RawValue, DeviceMetadata},
        storage::Log,
        helpers::Deferrable
    };

    #[test]
    fn test_push() {
        let metadata = DeviceMetadata::default();
        let log = Log::new(metadata.id, None).deferred();

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = RawValue::Binary(true);

        let routine = Routine::new(timestamp, metadata, value, log, command);

        let mut scheduled = SchedRoutineHandler::default();
        assert_eq!(0, scheduled.scheduled().into_iter().count());

        scheduled.push(routine);
        assert_eq!(1, scheduled.scheduled().into_iter().count());

        // Add second routine
        let metadata = DeviceMetadata::default();
        let log = Log::new(metadata.id, None).deferred();

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = RawValue::Binary(true);

        let routine = Routine::new(timestamp, metadata, value, log, command);

        scheduled.push(routine);
        assert_eq!(2, scheduled.scheduled().into_iter().count());

    }

    #[test]
    fn test_attempt() {
        let metadata = DeviceMetadata::default();
        let log = Log::new(metadata.id, None).deferred();

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = RawValue::Binary(true);

        let routine = Routine::new(timestamp, metadata, value, log.clone(), command);

        let mut scheduled = SchedRoutineHandler::default();

        scheduled.push(routine);

        // Add second routine
        let metadata = DeviceMetadata::default();
        let log = Log::new(metadata.id, None).deferred();

        let command = IOCommand::Output(|_| Ok(()));

        // BUG: why does this operation fail with any value less than 31 microseconds? There seems
        // to be a race condition.
        let ts2 = Utc::now() + Duration::microseconds(60);
        let value = RawValue::Binary(true);

        let routine = Routine::new(ts2, metadata, value, log.clone(), command);
        scheduled.push(routine);

        while Utc::now() < timestamp {
            assert_eq!(2, scheduled.scheduled().into_iter().count());
            scheduled.attempt();
        }
        scheduled.attempt();
        while Utc::now() < ts2 {
            assert_eq!(1, scheduled.scheduled().into_iter().count());
            scheduled.attempt();
        }
        scheduled.attempt();
        assert_eq!(0, scheduled.scheduled().into_iter().count());
    }
}
