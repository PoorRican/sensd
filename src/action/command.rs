use crate::io::IOEvent;


pub type CommandType = Box<dyn Command>;

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self) -> Option<IOEvent>;
}
