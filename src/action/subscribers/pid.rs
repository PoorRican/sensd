use crate::action::{PublisherInstance, Subscriber};
use crate::helpers::Deferred;
use crate::io::{DeviceType, IOEvent, RawValue};

/// Subscriber routine abstracting a PID controller
pub struct PIDMonitor {
    name: String,
    _threshold: RawValue,
    publisher: Option<Deferred<PublisherInstance>>,

    // TODO: check that device is output
    _output: Deferred<DeviceType>,
}

impl Subscriber for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
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
