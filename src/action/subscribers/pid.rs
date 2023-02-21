use crate::action::{ThresholdMonitor, PublisherInstance, SubscriberStrategy};
use crate::helpers::Deferred;
use crate::io::{DeviceType, IOEvent, IOType};

/// Subscriber routine to actively maintain an arbitrary threshold using PID
pub struct PIDMonitor {
    name: String,
    threshold: IOType,
    publisher: Option<Deferred<PublisherInstance>>,

    // TODO: check that device is output
    output: Deferred<DeviceType>,
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
