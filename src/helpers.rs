use std::fs::File;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::errors::Result;

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
/// This does not crash the program but instead prints any errors via `dbg!`.
pub fn check_results<T>(results: &[Result<T>]) -> Result<()> {
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
