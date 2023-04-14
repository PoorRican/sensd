use crate::action::{PublisherInstance, Subscriber, SubscriberType, EvaluationFunction};
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, RawValue};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
/// Controls when comparison of external value and threshold returns `true`.
///
/// Used by `Subscriber::evaluate()`
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

/// Perform an action if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdAction<F>
where
    F: FnMut(RawValue) -> Result<IOEvent, ErrorType>
{
    name: String,
    threshold: RawValue,
    publisher: Option<Deferred<PublisherInstance>>,

    trigger: Comparison,
    command: Option<F>,
    evaluator: EvaluationFunction,
}

impl<F> ThresholdAction<F>
where
    F: FnMut(RawValue) -> Result<IOEvent, ErrorType>
{
    pub fn new(
        name: String,
        threshold: RawValue,
        trigger: Comparison,
        command: Option<F>,
        evaluator: EvaluationFunction,
    ) -> Self {
        Self {
            name,
            threshold,
            publisher: None,
            trigger,
            command,
            evaluator,
        }
    }

    pub fn threshold(&self) -> RawValue {
        self.threshold
    }
}

impl<F> Subscriber for ThresholdAction<F>
where
    F: FnMut(RawValue) -> Result<IOEvent, ErrorType>
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn evaluate(&mut self, data: &IOEvent) {
        let value = data.data.value;
        let exceeded = match &self.trigger {
            &Comparison::GT => value > self.threshold,
            &Comparison::GTE => value >= self.threshold,
            &Comparison::LT => value < self.threshold,
            &Comparison::LTE => value <= self.threshold,
        };
        if exceeded {
            let EvaluationFunction::Threshold(evaluator) = self.evaluator;
            let output = (evaluator)(self.threshold, value);

            let msg = format!("{} {} {}", value, &self.trigger, self.threshold);
            self.notify(msg.as_str());

            if let Some(command) = &mut self.command {
                let _ = (command)(output);
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

impl<F> Deferrable for ThresholdAction<F>
where
    F: FnMut(RawValue) -> Result<IOEvent, ErrorType> + 'static
{
    type Inner = SubscriberType;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
