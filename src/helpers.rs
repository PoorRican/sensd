use std::fs::{create_dir_all, File};
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, TryLockResult};

use crate::errors::ErrorType;

/// Return a writable `File` from a given path.
///
/// If file or directory structure does not exist, then an attempt is made to create both.
pub fn writable_or_create<P>(path: P) -> File
where P: AsRef<Path>
{
    File::options()
        .write(true)
        .open(path.as_ref())
        // if an error occurs when reading, create file
        .unwrap_or_else(move |_| {
            match File::create(path.as_ref()) {
                Ok(_) => (),
                Err(_) => {
                    let parent = path.as_ref().parent().unwrap();
                    create_dir_all(parent).expect("Could not create root data directory");
                    File::create(&path).unwrap();
                }
            }
            File::options().write(true).open(path.as_ref()).unwrap()
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

/// Facade for an Arc wrapped around a Mutex with generic type T.
pub struct Def<T: Sized>(Arc<Mutex<T>>);
impl<T> Def<T> {
    pub fn new(deferred: T) -> Self {
        Self(Arc::new(Mutex::new(deferred)))
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, PoisonError<MutexGuard<T>>> {
        self.0.lock()
    }

    pub fn try_lock(&self) -> TryLockResult<MutexGuard<T>> {
        self.0.try_lock()
    }
}

impl<T: Default> Default for Def<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Def<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<Arc<Mutex<T>>> for Def<T> {
    fn from(value: Arc<Mutex<T>>) -> Def<T> {
        Def(value)
    }
}

impl<T> Into<Arc<Mutex<T>>> for Def<T> {
    fn into(self) -> Arc<Mutex<T>> {
        self.0
    }
}
