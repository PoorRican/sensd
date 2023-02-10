use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::errors::Result;
use crate::io::{IdType, InputType, MockPhSensor};
use crate::storage::LogType;

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
pub(crate) fn input_log_builder(name: &str, id: IdType) -> (Deferred<LogType>, Deferred<InputType>) {
    let log = Arc::new(Mutex::new(LogType::new()));
    let sensor = MockPhSensor::new(name.to_string(), id, log.clone());

    let wrapped = sensor.deferred();


    (log, wrapped)
}
