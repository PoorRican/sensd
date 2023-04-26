use crate::action::{PublisherInstance, Subscriber, SubscriberType, EvaluationFunction};
use crate::errors::{ErrorType, Error, ErrorKind};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, RawValue, DeferredDevice, DeviceType};
use std::fmt::{Display, Formatter};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

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

/// Subscriber that writes to output if threshold is exceeded.
#[derive(Clone)]
pub struct ThresholdAction {
    name: String,
    threshold: RawValue,
    publisher: Option<Deferred<PublisherInstance>>,

    trigger: Comparison,
    evaluator: EvaluationFunction,
    output: Option<DeferredDevice>,
}

impl ThresholdAction {
    /// Initialize a blank `ThresholdAction` without an associated publisher.
    pub fn new(
        name: String,
        threshold: RawValue,
        trigger: Comparison,
        output: Option<DeferredDevice>,
        evaluator: EvaluationFunction,
    ) -> Self {

        Self {
            name,
            threshold,
            publisher: None,
            trigger,
            output,
            evaluator,
        }
    }

    /// Getter for internal `threshold` value
    pub fn threshold(&self) -> RawValue {
        self.threshold
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

    /// Write to output if threshold is exceeded.
    ///
    /// `EvaluationFunction::Threshold` is used to determine output value.
    fn evaluate(&mut self, data: &IOEvent) {
        let input = data.data.value;
        let exceeded = match &self.trigger {
            &Comparison::GT =>  input > self.threshold,
            &Comparison::GTE => input >= self.threshold,
            &Comparison::LT =>  input < self.threshold,
            &Comparison::LTE => input <= self.threshold,
        };
        if exceeded {
            let EvaluationFunction::Threshold(evaluator) = self.evaluator;
            let output = (evaluator)(self.threshold, input);

            let msg = format!("{} {} {}", input, &self.trigger, self.threshold);
            self.notify(msg.as_str());

            if let Some(_) = self.output {
                self.write(output).unwrap();
            }
        }
    }

    fn publisher(&self) -> &Option<Deferred<PublisherInstance>> {
        &self.publisher
    }

    /// Assign publisher if field is `None`.
    ///
    /// Silently fails if publisher is already populated.
    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) {
        match self.publisher {
            None => self.publisher = Some(publisher),
            Some(_) => (),
        }
    }
}

impl Deferrable for ThresholdAction {
    type Inner = SubscriberType;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
