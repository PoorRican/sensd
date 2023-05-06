use std::fmt::{Display, Formatter};

#[allow(unused_imports)]
use crate::action::Action;

#[derive(Debug, Clone)]
/// Discrete variants that abstract comparison of external and threshold values.
///
/// External value should be always be on the left-side; internal threshold should be on the right side.
/// Internal command should be executed when this inequality returns true.
///
/// Used by [`Action::evaluate()`]
pub enum Comparison {
    GT,
    LT,
    GTE,
    LTE,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Comparison::GT => ">",
            Comparison::LT => "<",
            Comparison::GTE => "≥",
            Comparison::LTE => "≤",
        };
        write!(f, "{}", name)
    }
}
