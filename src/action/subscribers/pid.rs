use crate::action::{ThresholdMonitor, PublisherInstance, SubscriberStrategy};
use crate::helpers::Deferred;
use crate::io::{IOEvent, IOType, OutputType, };

/// Subscriber routine to actively maintain an arbitrary threshold using PID
pub struct PIDMonitor {
    name: String,
    threshold: IOType,
    publisher: Deferred<PublisherInstance>,

    output: Deferred<OutputType>,
}

impl ThresholdMonitor for PIDMonitor {
    fn threshold(&self) -> IOType {
        self.threshold
    }
}

impl SubscriberStrategy for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent> {
        todo!()
        // maintain PID
    }

    fn publisher(&self) -> Deferred<PublisherInstance> {
        self.publisher.clone()
    }
}
