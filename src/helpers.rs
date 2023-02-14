use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::errors::Result;
use crate::io::{Device, IdType, InputType, IOKind, GenericInput};
use crate::settings::Settings;
use crate::storage::OwnedLog;

/// Return a writable `File` from a given path.
/// File does not exist, then an attempt is made to create the file.
pub fn writable_or_create(path: String) -> File {
    File::options()
        .write(true)
        .open(path.deref())
        .unwrap_or_else(move |_| {
            File::create(path.deref()).unwrap();
            File::options().write(true).open(path.deref()).unwrap()
        })
}

/// Check a sequence of `Result`
/// This used to check the returned outputs of recursive or parallel operations.
pub fn check_results(results: &[Result<()>]) -> Result<()> {
    for result in results {
        match result {
            Err(e) => dbg!(e),
            _ => continue,
        };
    }
    Ok(())
}

// Defines a type alias `Deferred` for an Arc wrapped around a Mutex with generic type T.
pub type Deferred<T> = Arc<Mutex<T>>;

pub trait Deferrable {
    type Inner;
    fn deferred(self) -> Deferred<Self::Inner>;
}

/// Init sensor and `OwnedLog`, then set owner on log. Return log and sensor.
pub fn input_log_builder(
    name: &str,
    id: &IdType,
    kind: &Option<IOKind>,
    settings: Option<Arc<Settings>>,
) -> (Deferred<OwnedLog>, Deferred<InputType>) {
    let log = Arc::new(Mutex::new(OwnedLog::new(*id, settings)));
    let input = GenericInput::new(name.to_string(), *id, *kind, log.clone());

    let wrapped = input.deferred();
    log.lock().unwrap().set_owner(wrapped.clone());

    (log, wrapped)
}
