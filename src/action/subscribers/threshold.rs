use crate::action::{PublisherInstance, Subscriber, SubscriberType, EvaluationFunction};
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, IOType};
use std::sync::{Arc, Mutex};

/// Generic command that monitors a threshold
pub trait ThresholdMonitor: Subscriber {
    fn threshold(&self) -> IOType;
}

#[derive(Debug, Clone)]
/// Enum used by `ThresholdMonitor` logic
/// Controls when comparison of external value and threshold returns `true`.
pub enum Comparison {
    GT,
    LT,
}

/// Perform an action if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdAction<F>
where
    F: FnMut(IOType) -> Result<IOEvent, ErrorType>
{
    name: String,
    threshold: IOType,
    publisher: Option<Deferred<PublisherInstance>>,

    trigger: Comparison,
    command: Option<F>,
    evaluator: EvaluationFunction,
}

impl<F> ThresholdAction<F>
where
    F: FnMut(IOType) -> Result<IOEvent, ErrorType>
{
    pub fn new(
        name: String,
        threshold: IOType,
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
}

impl<F> ThresholdMonitor for ThresholdAction<F>
where
    F: FnMut(IOType) -> Result<IOEvent, ErrorType>
{
    fn threshold(&self) -> IOType {
        self.threshold
    }
}

impl<F> Subscriber for ThresholdAction<F>
where
    F: FnMut(IOType) -> Result<IOEvent, ErrorType>
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn evaluate(&mut self, data: &IOEvent) {
        let value = data.data.value;
        let exceeded = match &self.trigger {
            &Comparison::GT => value >= self.threshold,
            &Comparison::LT => value <= self.threshold,
        };
        if exceeded {
            let EvaluationFunction::Threshold(evaluator) = self.evaluator;
            let output = (evaluator)(self.threshold, value);


            let msg = format!("{} exceeds {}", value, self.threshold);
            self.notify(msg.as_str());

            if let Some(command) = &mut self.command {
                let _ = (command)(output);
            }
        }
    }

    fn publisher(&self) -> &Option<Deferred<PublisherInstance>> {
        &self.publisher
    }

    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) {
        match self.publisher {
            None => self.publisher = Some(publisher),
            Some(_) => (),
        }
    }
}

impl<F> Deferrable for ThresholdAction<F>
where
    F: FnMut(IOType) -> Result<IOEvent, ErrorType> + 'static
{
    type Inner = SubscriberType;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
