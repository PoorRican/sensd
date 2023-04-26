use std::fs::{File, create_dir_all};
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::errors::ErrorType;

/// Return a writable `File` from a given path.
///
/// If file or directory structure does not exist, then an attempt is made to create both.
pub fn writable_or_create(path: String) -> File {
    File::options()
        .write(true)
        .open(path.deref())
        // if an error occurs when reading, create file
        .unwrap_or_else(move |_| {
            match File::create(path.deref()) {
                Ok(_) => (),
                Err(_) => {
                    let _path = Path::new(&path);
                    let parent = _path.parent().unwrap();
                    create_dir_all(parent).expect("Could not create root data directory");
                    File::create(_path).unwrap();
                }
            }
            File::options().write(true).open(path.deref()).unwrap()
        })
}

/// Check a sequence of `Result`
/// This used to check the returned outputs of recursive or parallel operations.
/// This does not crash the program but instead prints any errors via `dbg!`.
pub fn check_results<T>(results: &[Result<T, ErrorType>]) -> Result<(), ErrorType> {
    for result in results {
        match result {
            Err(e) => eprintln!("█▓▒░ ERROR: {}", e),
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
