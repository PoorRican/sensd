use crate::action::Routine;

#[allow(unused_imports)]
use crate::storage::Group;

#[derive(Default)]
/// Wrapper for a collection of scheduled [`Routine`] instances that handles real-time execution
/// Self-contained collection of scheduled [`Routine`]s for a single [`crate::action::Publisher`].
///
/// This struct acts as a facade for an arbitrary collection (in this case, [`Vec`]).
pub struct SchedRoutineHandler(Vec<Routine>);

impl SchedRoutineHandler {
    /// Push a new [`Routine`] to internal collection
    ///
    /// # Parameters
    ///
    /// - `routine`: `Routine` to add to internal collection
    pub fn push(&mut self, routine: Routine) {
        self.0.push(routine)
    }

    /// Attempt to execute scheduled routines.
    ///
    /// Even though [`Routine`] instances are scheduled during normal polling cycles
    /// by [`Group`], the assumption is that their scheduled execution time does not
    /// correlate with polling interval. Therefore, [`SchedRoutineHandler::attempt_routines()`]
    /// should be called as often as possible, and outside of normal polling cycle,
    /// to produce a real-time response.
    ///
    /// Any routines executed by [`Routine::attempt()`] are cleared from the internal container.
    pub fn attempt_routines(&mut self) {
        let mut executed = Vec::default();
        for (index, routine) in self.0.iter().enumerate() {
            if routine.attempt() {
                executed.push(index);
            }
        }
        // remove completed routines
        for index in executed {
            self.0.remove(index);
        }
    }

    /// Getter function for internal collection
    ///
    /// # Returns
    ///
    /// Slice of [`Routine`]
    pub fn scheduled(&self) -> &[Routine] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use crate::{
        action::{IOCommand, Routine, SchedRoutineHandler},
        helpers::Def,
        io::{DeviceMetadata, Datum},
        storage::Log,
    };

    #[test]
    fn test_push() {
        let metadata = DeviceMetadata::default();
        let log = Def::new(Log::with_metadata(&metadata));

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = Datum::Binary(true);

        let routine = Routine::new(timestamp, value, log, command);

        let mut scheduled = SchedRoutineHandler::default();
        assert_eq!(0, scheduled.scheduled().into_iter().count());

        scheduled.push(routine);
        assert_eq!(1, scheduled.scheduled().into_iter().count());

        // Add second routine
        let metadata = DeviceMetadata::default();
        let log = Def::new(Log::with_metadata(&metadata));

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(5);
        let value = Datum::Binary(true);

        let routine = Routine::new(timestamp, value, log, command);

        scheduled.push(routine);
        assert_eq!(2, scheduled.scheduled().into_iter().count());
    }

    #[test]
    /// Sometimes this fails due to race condition mentioned below (issue #95). In that case,
    /// running the tests again should pass.
    fn test_attempt() {
        let metadata = DeviceMetadata::default();
        let log = Def::new(Log::with_metadata(&metadata));

        let command = IOCommand::Output(|_| Ok(()));

        let timestamp = Utc::now() + Duration::microseconds(30);
        let value = Datum::Binary(true);

        let routine = Routine::new(timestamp, value, log.clone(), command);

        let mut scheduled = SchedRoutineHandler::default();

        scheduled.push(routine);

        // Add second routine
        let metadata = DeviceMetadata::default();
        let log = Def::new(Log::with_metadata(&metadata));

        let command = IOCommand::Output(|_| Ok(()));

        // BUG: why does this operation fail with any value less than 31 microseconds? There seems
        // to be a race condition.
        let ts2 = Utc::now() + Duration::microseconds(120);
        let value = Datum::Binary(true);

        let routine = Routine::new(ts2, value, log.clone(), command);
        scheduled.push(routine);

        while Utc::now() < timestamp {
            assert_eq!(2, scheduled.scheduled().into_iter().count());
            scheduled.attempt_routines();
        }
        scheduled.attempt_routines();
        while Utc::now() < ts2 {
            assert_eq!(1, scheduled.scheduled().into_iter().count());
            scheduled.attempt_routines();
        }
        scheduled.attempt_routines();
        assert_eq!(0, scheduled.scheduled().into_iter().count());
    }
}
