use crate::action::{Action, BoxedAction};
use crate::io::{IOEvent, Output, RawValue};
use crate::action::trigger::Trigger;
use crate::helpers::Def;

/// Bang-bang (on-off) controller
///
/// If threshold is exceeded, a notification is printed and output is actuated until next polling cycle
/// where input value is below threshold. In the future, upper and lower thresholds will be added for
/// finer control.
///
/// Unlike the [`crate::action::PIDMonitor`] subscriber, [`Threshold`] is unable create a [`Routine`].
/// Instead, functionality implements simple on/off behavior.
///
/// # Usage
///
/// ## Reservoir Fill Level
///
/// Given a reservoir, with a sensor for reading fill level, a pump for increasing fill level, and a
/// valve that decreases fill level. Two separate [`Threshold`] could be used for controlling
/// this system based off of input from the level sensor. Depending on polling frequency there might be
/// some variance between threshold value and the input value when actuation stops.
// TODO: add upper/lower threshold
pub struct Threshold {
    name: String,
    threshold: RawValue,

    trigger: Trigger,
    output: Option<Def<Output>>,
}

impl Threshold {
    /// Constructor for [`Threshold`]
    ///
    /// # Parameters
    ///
    /// - `name`: name of action
    /// - `threshold`: Threshold that controls what external value actuates/de-actuates device
    /// - `trigger`: Defines the relationship between threshold and external value.
    ///
    /// # Returns
    /// Initialized [`Threshold`] action without `output` set.
    ///
    /// **Note**: [`Action::set_output()`] builder function should be chained after initialization.
    ///
    /// # See Also
    ///
    /// - [`Action::with_output()`] for constructor that accepts an `output` parameter.
    // TODO: there should be an option to inverse polarity
    pub fn new(name: String, threshold: RawValue, trigger: Trigger) -> Self {
        // TODO: add a type check to ensure that `output` accepts a binary value

        Self {
            name,
            threshold,
            trigger,
            output: None,
        }
    }

    /// Constructor that accepts `output` parameter
    ///
    /// # Parameters
    ///
    /// - `name`: name of action
    /// - `threshold`: Threshold that controls what external value actuates/de-actuates device
    /// - `trigger`: Defines the relationship between threshold and external value.
    /// - `output`: Output device
    ///
    /// # Returns
    ///
    /// Initialized [`Threshold`] action with `output` set.
    pub fn with_output(name: String, threshold: RawValue, trigger: Trigger, output: Def<Output>) -> Self {
        Self::new(name, threshold, trigger).set_output(output)
    }

    #[inline]
    /// Getter for internal `threshold` value
    pub fn threshold(&self) -> RawValue {
        self.threshold
    }

    #[inline]
    /// Actuate output device without runtime validation
    ///
    /// Sends a `true` value to output device. Does not check value [`Result`] from [`Action::write()`].
    fn on_unchecked(&self) {
        let _ = self.write(RawValue::Binary(true));
    }

    #[inline]
    /// De-actuate output device without runtime validation
    ///
    /// Sends a `false` value to output device. Does not check value [`Result`] from [`Action::write()`].
    fn off_unchecked(&self) {
        let _ = self.write(RawValue::Binary(false));
    }
}

impl Action for Threshold {
    #[inline]
    /// Name of action
    fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    /// Evaluate external data
    ///
    /// Incoming data is compared against internal threshold using [`Trigger::exceeded()`]. If
    /// incoming data exceeds threshold, output device is actuated. Otherwise, output device is
    /// deactivated.
    ///
    /// # Notes
    ///
    /// - This function is inline because it is used in iterator loops
    /// - Any error returned by [`Self::write()`] is dropped by [`Self::on_unchecked()`] and
    ///   [`Self::off_unchecked()`]
    fn evaluate(&mut self, data: &IOEvent) {
        let input = data.data.value;
        let exceeded = self.trigger.exceeded(input, self.threshold);

        match exceeded {
            true => {
                // Notify if exceeded
                let msg = format!("{} {} {}", input, &self.trigger, self.threshold);
                self.notify(msg.as_str());

                self.on_unchecked();
            },
            false => { self.off_unchecked() },
        };
    }

    fn set_output(mut self, device: Def<Output>) -> Self
    where
        Self: Sized,
    {
        self.output = Some(device);

        self
    }

    #[inline]
    fn output(&self) -> Option<Def<Output>> {
        self.output.clone()
    }

    #[inline]
    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
