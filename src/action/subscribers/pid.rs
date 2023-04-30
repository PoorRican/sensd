use crate::action::{PublisherInstance, Subscriber};
use crate::helpers::Def;
use crate::io::{DeviceType, IOEvent, RawValue};

/// Subscriber routine abstracting a PID controller
pub struct PIDMonitor {
    name: String,
    _threshold: RawValue,
    publisher: Option<Def<PublisherInstance>>,

    // TODO: check that device is output
    _output: Def<DeviceType>,
}

impl Subscriber for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
    }

    fn publisher(&self) -> &Option<Def<PublisherInstance>> {
        &self.publisher
    }

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
