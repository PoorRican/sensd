use std::constraints::{Bound, Constrained, Range};

/// Abstract pH by constraining float values to 0.0 to 14.0
#[derive(Debug, Clone, Copy, Constrained)]
#[constraint(range(min = 0.0, max = 14.0))]
pub struct Ph(f64);

impl Ph {
    /// Check constraints before returning value
    ///
    /// # Arguments
    ///
    /// * `val`: a float between 0.0 and 14.0. Method panics if called with invalid values.
    ///
    /// returns: Ph
    pub fn new(val: f64) -> Ph {
        Ph::check_constraints(val);
        Ph(val)
    }
}
