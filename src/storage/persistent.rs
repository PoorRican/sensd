use crate::errors::ErrorType;

/// Default filetype suffix.
///
/// Used to generate filenames.
pub const FILETYPE: &str = ".json";

/// Expresses an interface to save or load from disk
pub trait Persistent {
    /// save data to disk
    fn save(&self) -> Result<(), ErrorType>;

    /// load from disk
    fn load(&mut self) -> Result<(), ErrorType>;
}
