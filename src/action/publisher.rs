//! Implements a control system based off of evaluating incoming data.

use crate::action::{BoxedAction, SchedRoutineHandler};
use crate::helpers::Def;
use crate::io::IOEvent;

#[derive(Default)]
/// Handles storage and association between an [`Input`] and [`crate::action::Action`] instances
///
/// [`Publisher`] uses the [observer design pattern](https://refactoring.guru/design-patterns/observer)
/// to disseminate [`IOEvent`]'s to [`crate::action::Action`] subscribers. Only one [`Publisher`] can
/// be assigned to an [`Input`], but any number of subscribers are allowed. When data is read from an
/// [`Input`], the generated [`IOEvent`] is passed to to all subscribers by [`Publisher::propagate()`].
///
/// Additionally, [`Publisher`] maintains the internal collection of scheduled [`crate::action::Routine`]s
/// for any number of output devices and provides [`Publisher::attempt_routines()`] for executing those
/// scheduled commands at their scheduled time.
pub struct Publisher {
    actions: Vec<BoxedAction>,
    scheduled: Def<SchedRoutineHandler>,
}

impl Publisher {
    #[inline]
    /// Attempt to run scheduled [`crate::action::Routine`]s.
    ///
    /// [`crate::action::Routine`] instances are automatically added by internal
    /// [`crate::action::Action`]s and are automatically cleared when executed.
    ///
    /// # See Also
    ///
    /// This is a facade for [`SchedRoutineHandler::attempt_routines()`], which contains more
    /// detailed notes.
    ///
    /// # Panics
    ///
    /// Panic is thrown if [`SchedRoutineHandler`] cannot be locked.
    pub fn attempt_routines(&mut self) {
        self.scheduled.try_lock().unwrap().attempt_routines()
    }

    /// Get collection of subscribed [`crate::action::Action`]'s (stored as [`BoxedAction`]).
    ///
    /// # Returns
    ///
    /// Slice of all [`BoxedAction`] associated with `self`
    pub fn subscribers(&self) -> &[BoxedAction] {
        &self.actions
    }

    /// Add [`crate::action::Action`] to internal collection.
    ///
    /// # Parameters
    ///
    /// - `subscriber`: [`BoxedAction`] to add to internal store.
    pub fn subscribe(&mut self, subscriber: BoxedAction) {
        self.actions.push(subscriber)
    }

    /// Handle incoming data
    ///
    /// [`crate::action::Action::evaluate()`] is called on all associated
    /// [`crate::action::Action`] instances and incoming data is passed.
    ///
    /// # Parameters
    ///
    /// - `data`: Incoming [`IOEvent`] generated from [`crate::io::Input::read()`]
    pub fn propagate(&mut self, data: &IOEvent) {
        for subscriber in self.actions.iter_mut() {
            subscriber.evaluate(data);
        }
    }

    /// Method to get passable reference to internal handler
    ///
    /// This is used when an [`crate::action::Action`] needs to schedule
    /// [`crate::action::Routine`] (ie: in the case of [`crate::action::actions::PID`])
    ///
    /// # Returns
    ///
    /// Reference to [`SchedRoutineHandler`] guarded by [`Def`]
    pub fn handler_ref(&self) -> Def<SchedRoutineHandler> {
        self.scheduled.clone()
    }
}
