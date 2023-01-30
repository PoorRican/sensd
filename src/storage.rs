// trait that expresses an interface to save or load from disk
pub trait Persistent {
    // save data to disk
    fn save(&self) -> bool;

    // load from disk
    fn load(&self) -> bool;
 }