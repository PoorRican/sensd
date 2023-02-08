use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::errors::Result;

pub fn write_or_create(path: &Path) -> File {
    File::options().write(true).open(path)
        .unwrap_or_else(move |_| {
            File::create(path).unwrap();
            File::options().write(true).open(path).unwrap()
        })
}

pub fn check_results(results: &[Result<()>]) -> Result<()> {
    for &result in results.into_iter() {
        match result {
            Err(e) => return Err(e),
            _ => continue
        }
    };
    Ok(())
}

// Defines a type alias `Deferred` for an Arc wrapped around a Mutex with generic type T.
pub type Deferred<T> = Arc<Mutex<T>>;
