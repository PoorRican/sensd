use crate::helpers::Deferrable;
use crate::helpers::Deferred;
use crate::io::{IdType, InputDevice};
use crate::storage::OwnedLog;

pub trait Sensor: Default + InputDevice + Deferrable {
    fn new(name: String, sensor_id: IdType, log: Deferred<OwnedLog>) -> Self;
}
