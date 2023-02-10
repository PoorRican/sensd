use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::errors::Result;
use crate::io::{IdType, InputType, MockPhSensor, Device, InputDevice, DeviceType};
use crate::settings::Settings;
use crate::storage::OwnedLog;

pub fn write_or_create(path: String) -> File {
    File::options().write(true).open(path.deref())
        .unwrap_or_else(move |_| {
            File::create(path.deref()).unwrap();
            File::options().write(true).open(path.deref()).unwrap()
        })
}

pub fn check_results(results: &[Result<()>]) -> Result<()> {
    for result in results {
        match result {
            Err(e) => dbg!(e),
            _ => continue
        };
    };
    Ok(())
}

// Defines a type alias `Deferred` for an Arc wrapped around a Mutex with generic type T.
pub type Deferred<T> = Arc<Mutex<T>>;

pub trait Deferrable<T> {
    fn deferred(&self) -> Deferred<T>;
}

/// Init sensor and `OwnedLog`, then set owner on log. Return log and sensor.
pub fn input_log_builder(name: &str, id: IdType, settings: Arc<Settings>) -> (Deferred<OwnedLog>, Deferred<InputType>) {
    let log = Arc::new(Mutex::new(OwnedLog::new(id, settings)));
    let sensor = MockPhSensor::new(name.to_string(), id, log.clone());

    let wrapped = sensor.deferred();
    log.lock().unwrap().set_owner(wrapped.clone());


    (log, wrapped)
}
