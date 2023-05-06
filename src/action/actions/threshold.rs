use crate::action::{Action, BoxedAction};
use crate::errors::ErrorType;
use crate::io::{Output, IOEvent, RawValue};
use std::fmt::{Display, Formatter};
use crate::helpers::Def;

#[derive(Debug, Clone)]
/// Discrete variants that abstract comparison of external and threshold values.
///
/// External value should be always be on the left-side; internal threshold should be on the right side.
/// Internal command should be executed when this inequality returns true.
///
/// Used by [`Threshold::evaluate()`]
pub enum Comparison {
    GT,
    LT,
    GTE,
    LTE,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Comparison::GT => ">",
            Comparison::LT => "<",
            Comparison::GTE => "≥",
            Comparison::LTE => "≤",
        };
        write!(f, "{}", name)
    }
}

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

    trigger: Comparison,
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
    pub fn new(name: String, threshold: RawValue, trigger: Comparison) -> Self {
        // TODO: add a type check to `RawValue` to ensure a numeric value
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
    pub fn with_output(name: String, threshold: RawValue, trigger: Comparison, output: Def<Output>) -> Self {
        Self::new(name, threshold, trigger).set_output(output)
    }

    #[inline]
    /// Getter for internal `threshold` value
    pub fn threshold(&self) -> RawValue {
        self.threshold
    }

    #[inline]
    /// Actuate output device
    ///
    /// Sends a `true` value to output device
    ///
    /// # Returns
    /// - `Ok(IOEvent)`: when I/O operation completes successfully.
    /// - `Err(ErrorType)`: when an error occurs during I/O operation
    fn on(&self) -> Result<IOEvent, ErrorType> {
        self.write(RawValue::Binary(true))
    }

    #[inline]
    /// De-actuate output device.
    ///
    /// Sends a `false` value to output device
    ///
    /// # Returns
    /// - `Ok(IOEvent)`: when I/O operation completes successfully.
    /// - `Err(ErrorType)`: when an error occurs during I/O operation
    fn off(&self) -> Result<IOEvent, ErrorType> {
        self.write(RawValue::Binary(false))
    }
}

impl Action for Threshold {
    #[inline]
    /// Name of action
    fn name(&self) -> &String {
        &self.name
    }

    /// Evaluate external data
    ///
    /// Incoming data is compared against [`Threshold::threshold()`] using internal `trigger`.
    /// If incoming data exceeds threshold, output device is actuated. Otherwise, output device is deactivated.
    // TODO: check state cache of output device to avoid redundant calls to output device.
    fn evaluate(&mut self, data: &IOEvent) {
        let input = data.data.value;
        let exceeded = match &self.trigger {
            &Comparison::GT => input > self.threshold,
            &Comparison::GTE => input >= self.threshold,
            &Comparison::LT => input < self.threshold,
            &Comparison::LTE => input <= self.threshold,
        };
        if exceeded {
            let msg = format!("{} {} {}", input, &self.trigger, self.threshold);
            self.notify(msg.as_str());

            if let Some(_) = self.output {
                self.on().unwrap();
            }
        } else if let Some(_) = self.output {
            self.off().unwrap();
        }
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
