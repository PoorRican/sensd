use std::sync::{Arc, Mutex};
use crate::action::{BaseCommandFactory, Command, SimpleNotifier, PublisherInstance, SubscriberStrategy, SubscriberType};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, IOType};

/// Generic command that monitors a threshold
pub trait ThresholdMonitor: SubscriberStrategy {
    fn threshold(&self) -> IOType;
}

#[derive(Debug, Clone)]
/// Enum used by `ThresholdMonitor` logic
/// Controls when comparison of external value and threshold returns `true`.
pub enum Comparison {
    GT,
    LT,
}

/// Notify if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdNotifier {
    name: String,
    threshold: IOType,
    publisher: Option<Deferred<PublisherInstance>>,

    trigger: Comparison,
    factory: BaseCommandFactory,
}

impl ThresholdNotifier {
    pub fn new(
        name: String,
        threshold: IOType,
        trigger: Comparison,
        factory: BaseCommandFactory
    ) -> Self {
        Self {
            name,
            threshold,
            publisher: None,
            trigger,
            factory,
        }
    }
}

impl ThresholdMonitor for ThresholdNotifier {
    fn threshold(&self) -> IOType {
        self.threshold
    }
}

impl SubscriberStrategy for ThresholdNotifier {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn evaluate(&mut self, event: &IOEvent) -> Option<IOEvent> {
        let value = event.data.value;
        let exceed = match &self.trigger {
            &Comparison::GT => value >= self.threshold,
            &Comparison::LT => value <= self.threshold,
        };
        if exceed {
            // insert command here
            let msg = format!("{} exceeded {}", value, self.threshold);
            let command = SimpleNotifier::new(msg);
            // Some(event.invert(1.0))  // re-enable this when dynamic IOTypes have been implemented
            command.execute()
        } else {
            None
        }
    }

    fn publisher(&self) -> &Option<Deferred<PublisherInstance>> {
        &self.publisher
    }

    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>) {
        match self.publisher {
            None => self.publisher = Some(publisher),
            Some(_) => ()
        }
    }
}

impl Deferrable for ThresholdNotifier {
    type Inner = SubscriberType;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
