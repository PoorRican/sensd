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
/// Unlike the [`crate::action::actions::PID`] subscriber, [`Threshold`] is unable create a
/// [`crate::action::Routine`].
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
    /// # Example
    ///
    /// ```
    /// use sensd::io::RawValue;
    /// use sensd::action::{actions, Trigger};
    ///
    /// let action = actions::Threshold::new("", RawValue::Float(1.0), Trigger::GT);
    /// ```
    ///
    /// **Note**: [`Action::set_output()`] builder method should be chained after initialization.
    ///
    /// # See Also
    ///
    /// - [`Threshold::with_output()`] for constructor that accepts an `output` parameter.
    // TODO: there should be an option to inverse polarity
    pub fn new<N>(name: N, threshold: RawValue, trigger: Trigger) -> Self
    where
        N: Into<String>
    {
        // TODO: add a type check to ensure that `output` accepts a binary value

        Self {
            name: name.into(),
            threshold,
            trigger,
            output: None,
        }
    }

    /// Constructor that accepts `output` parameter
    ///
    /// This method can be called instead of using [`Threshold::new()`] followed by
    /// [`Threshold::set_output()`].
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
    ///
    /// # Example
    ///
    /// This method is meant to be used as a builder pattern via method chaining:
    ///
    /// ```
    /// use sensd::io::{Device, Output, RawValue};
    /// use sensd::action::{Action, actions, Trigger};
    ///
    /// let output = Output::default().into_deferred();
    /// let action = actions::Threshold::with_output("",
    ///                                         RawValue::Float(1.0),
    ///                                         Trigger::GT,
    ///                                         output);
    /// assert!(action.output().is_some())
    /// ```
    pub fn with_output<N>(name: N, threshold: RawValue, trigger: Trigger, output: Def<Output>) -> Self
    where
        N: Into<String>
    {
        Self::new(name.into(), threshold, trigger).set_output(output)
    }

    #[inline]
    /// Getter for internal `threshold` value
    ///
    /// # Returns
    ///
    /// Copy of internal [`RawValue`] to use as threshold
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::{Device, Output, RawValue};
    /// use sensd::action::{Action, actions, Trigger};
    ///
    /// let threshold = RawValue::Float(1.0);
    /// let output = Output::default().into_deferred();
    /// let action = actions::Threshold::new("", threshold, Trigger::GT);
    ///
    /// assert_eq!(threshold, action.threshold())
    /// ```
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
    /// - Any error returned by [`Self::write()`] is silenced.
    fn evaluate(&mut self, data: &IOEvent) {
        let input = data.value;
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

    ///
    /// Builder function for setting `output` field.
    ///
    /// # Parameters
    ///
    /// - `device`: [`Def`] reference to set as output
    ///
    /// # Returns
    ///
    /// Ownership of `Self` to enable method chaining
    ///
    /// # Example
    ///
    /// This method is meant to be used as a builder pattern via method chaining:
    ///
    /// ```
    /// use sensd::io::{Device, Output, RawValue};
    /// use sensd::action::{Action, actions, Trigger};
    ///
    /// let output = Output::default().into_deferred();
    /// let action = actions::Threshold::new("", RawValue::Float(1.0), Trigger::GT)
    ///                 .set_output(output);
    /// assert!(action.output().is_some())
    /// ```
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

#[cfg(test)]
mod tests {
    use crate::action::actions::Threshold;
    use crate::action::Trigger;
    use crate::io::{Device, Output, RawValue};

    #[test]
    /// Ensure that `name` can be given to `new()` constructor as `String` or `&str`
    fn new_name_parameter() {
        let name = "test name";
        Threshold::new(name, RawValue::default(), Trigger::GT);

        let name = String::from(name);
        Threshold::new(name, RawValue::default(), Trigger::GT);
    }

    #[test]
    /// Ensure that `name` can be given to `with_output()` constructor as `String` or `&str`
    fn with_output_name_parameter() {
        let output = Output::default().into_deferred();
        let name = "test name";
        Threshold::with_output(name, RawValue::default(), Trigger::GT, output.clone());

        let name = String::from(name);
        Threshold::with_output(name, RawValue::default(), Trigger::GT, output);
    }
}