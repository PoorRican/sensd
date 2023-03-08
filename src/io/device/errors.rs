use crate::errors::*;

pub fn no_internal_closure() -> Box<dyn std::error::Error> {
    Error::new(ErrorKind::CommandError, "Device has no internal closure")
}
