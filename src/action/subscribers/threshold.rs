use crate::action::{PublisherInstance, Subscriber};
use crate::errors::{ErrorType, Error, ErrorKind};
use crate::helpers::Def;
use crate::io::{IOEvent, RawValue, DeferredDevice, DeviceType};
use std::fmt::{Display, Formatter};
use std::ops::DerefMut;

#[derive(Debug, Clone)]
/// Controls when comparison of external value and threshold returns `true`.
///
/// Used by `ThresholdAction::evaluate()`
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

/// Subscriber that reacts in a binary fashion if threshold is exceeded.
///
/// If the threshold is exceeded, then the subscriber can be setup to notify, and/or actuate an
/// output in a binary fashion. The intention for this subscriber is not to be accurate, but
/// rather, provide loose control. Output device is actuated as long as threshold is exceeded. In
/// the future, upper and lower thresholds will be added for fine tuning of action execution.
///
/// Unlike the `PIDController` subscriber, `ThresholdAction` does not create a `Routine`. Instead,
/// it is intended to implement on/off behavior.
///
/// # Scenarios
///
/// ## Reservoir Fill Level
/// Given a reservoir, with a sensor for reading fill level, a pump for increasing fill level and a
/// valve for decreasing fill level. The refill pump could be set to turn on at 25% but might stop
/// when fill level reaches 30%. Likewise, the dump valve might be set to decrease fill level at 90%,
/// but dumping might stop at 80%.
///
// TODO: add upper/lower threshold
#[derive(Clone)]
pub struct ThresholdAction {
    name: String,
    threshold: RawValue,
    publisher: Option<Def<PublisherInstance>>,

    trigger: Comparison,
    output: Option<DeferredDevice>,
}

impl ThresholdAction {
    /// Initialize a blank `ThresholdAction`
    ///
    /// No `PublisherInstance` is associated
    pub fn new(
        name: String,
        threshold: RawValue,
        trigger: Comparison,
        output: Option<DeferredDevice>,
    ) -> Self {

        Self {
            name,
            threshold,
            publisher: None,
            trigger,
            output,
        }
    }

    /// Getter for internal `threshold` value
    pub fn threshold(&self) -> RawValue {
        self.threshold
    }

    fn on(&self) -> Result<IOEvent, ErrorType> {
        self.write(RawValue::Binary(true))
    }

    fn off(&self) -> Result<IOEvent, ErrorType> {
        self.write(RawValue::Binary(false))
    }

    /// Pass value to output device
    fn write(&self, value: RawValue) -> Result<IOEvent, ErrorType> {
        if let Some(inner) = self.output.clone() {
            let mut binding = inner.try_lock().unwrap();
            let device = binding.deref_mut();
            match device {
                DeviceType::Output(output) => output.write(value),
                _ => Err(Error::new(ErrorKind::DeviceError,
                                    "Associated output device is misconfigured."))
            }
        } else {
            Err(Error::new(ErrorKind::DeviceError,
                           "ThresholdAction has no device associated as output."))
        }
    }

}

impl Subscriber for ThresholdAction {
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Turn on output device if threshold is exceeded.
    ///
    // TODO: there should be an option to inverse polarity
    // TODO: there should be a check to ensure that output device is binary
    // TODO: should there be a state check of output device?
    fn evaluate(&mut self, data: &IOEvent) {
        let input = data.data.value;
        let exceeded = match &self.trigger {
            &Comparison::GT =>  input > self.threshold,
            &Comparison::GTE => input >= self.threshold,
            &Comparison::LT =>  input < self.threshold,
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

    fn publisher(&self) -> &Option<Def<PublisherInstance>> {
        &self.publisher
    }

    /// Assign publisher if field is `None`.
    ///
    /// Silently fails if publisher is already populated.
    fn add_publisher(&mut self, publisher: Def<PublisherInstance>) {
        match self.publisher {
            None => self.publisher = Some(publisher),
            Some(_) => (),
        }
    }

    fn into_subscriber(self) -> Box<dyn Subscriber> {
        Box::new(self)
    }
}
