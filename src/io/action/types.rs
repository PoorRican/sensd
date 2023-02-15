use crate::helpers::Deferred;
use crate::io::{Command, IOType};

pub type CommandType = Box<dyn Command>;

pub type BaseCommandFactory<T, Z> = fn(T, Z) -> CommandType;