use core::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// Enum used to classify direction of data flow in relation to system.
///
/// # Variants
///
/// - `In`: indicates that data came from the outside world. This is the default.
/// - `Out`: indicates that accept data was sent to manipulate and represents
///   physical/tangible change.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
pub enum IODirection {
    #[default]
    In,
    Out,
}

impl Display for IODirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IODirection::In => "Input",
            IODirection::Out => "Output",
        };
        write!(f, "{}", name)
    }
}
