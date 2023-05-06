use crate::errors::ErrorType;

/// Default filetype suffix.
///
/// Used by [`Log::filename()`]
pub const FILETYPE: &str = ".json";

/// Expresses an interface to save or load from disk
pub trait Persistent {
    /// save data to disk
    fn save(&self, path: &Option<String>) -> Result<(), ErrorType>;

    /// load from disk
    fn load(&mut self, path: &Option<String>) -> Result<(), ErrorType>;
}
