use crate::errors::Error;
use float_cmp::approx_eq;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Type used for passing between IO abstractions.
///
/// An enum is used to avoid a generic implementation of [`crate::storage::Log`] caused by
/// a generic implementation of [`crate::io::IOEvent`].
///
/// # Notes
/// The implemented types have been chosen as a good fit for GPIO. However,
/// if a type is needed that is not here, feel free to initiate a pull request.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialOrd)]
pub enum RawValue {
    Binary(bool),
    PosInt8(u8),
    Int8(i8),
    PosInt(u32),
    Int(i32),
    Float(f32),
}

impl RawValue {
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Binary(_) => false,
            _ => true,
        }
    }
}

impl Default for RawValue {
    fn default() -> Self {
        RawValue::PosInt8(u8::default())
    }
}

impl Display for RawValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Binary(val) => {
                    if *val { "true" } else { "false" }.to_string()
                }
                Self::PosInt8(val) => val.to_string(),
                Self::Int8(val) => val.to_string(),
                Self::PosInt(val) => val.to_string(),
                Self::Int(val) => val.to_string(),
                Self::Float(val) => val.to_string(),
            }
        )
    }
}

// █▓▒░ Conversion from primitive types
impl TryFrom<u8> for RawValue {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(RawValue::PosInt8(value))
    }
}
impl TryFrom<i8> for RawValue {
    type Error = Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(RawValue::Int8(value))
    }
}
impl TryFrom<u32> for RawValue {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(RawValue::PosInt(value))
    }
}
impl TryFrom<i32> for RawValue {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(RawValue::Int(value))
    }
}
impl TryFrom<f32> for RawValue {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(RawValue::Float(value))
    }
}
impl TryFrom<bool> for RawValue {
    type Error = Error;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(RawValue::Binary(value))
    }
}

// █▓▒░ Basic mathematical operations
impl Add for RawValue {
    type Output = RawValue;

    fn add(self, other: RawValue) -> RawValue {
        match (self, other) {
            (RawValue::Binary(x), RawValue::Binary(y)) => RawValue::Binary(x || y),
            (RawValue::Float(x), RawValue::Float(y)) => RawValue::Float(x + y),
            (RawValue::Int8(x), RawValue::Int8(y)) => RawValue::Int8(x + y),
            (RawValue::PosInt8(x), RawValue::PosInt8(y)) => RawValue::PosInt8(x + y),
            (RawValue::Int(x), RawValue::Int(y)) => RawValue::Int(x + y),
            (RawValue::PosInt(x), RawValue::PosInt(y)) => RawValue::PosInt(x + y),
            _ => panic!("Cannot add mismatched RawValue types"),
        }
    }
}

impl Sub for RawValue {
    type Output = RawValue;

    fn sub(self, other: RawValue) -> RawValue {
        // TODO: Catch binary as type
        match (self, other) {
            (RawValue::Float(x), RawValue::Float(y)) => RawValue::Float(x - y),
            (RawValue::Int8(x), RawValue::Int8(y)) => RawValue::Int8(x - y),
            (RawValue::PosInt8(x), RawValue::PosInt8(y)) => RawValue::PosInt8(x - y),
            (RawValue::Int(x), RawValue::Int(y)) => RawValue::Int(x - y),
            (RawValue::PosInt(x), RawValue::PosInt(y)) => RawValue::PosInt(x - y),
            _ => panic!("Cannot subtract mismatched RawValue types"),
        }
    }
}

impl Mul for RawValue {
    type Output = RawValue;

    fn mul(self, other: RawValue) -> RawValue {
        // TODO: Catch binary as type
        match (self, other) {
            (RawValue::Float(x), RawValue::Float(y)) => RawValue::Float(x * y),
            (RawValue::Int8(x), RawValue::Int8(y)) => RawValue::Int8(x * y),
            (RawValue::PosInt8(x), RawValue::PosInt8(y)) => RawValue::PosInt8(x * y),
            (RawValue::Int(x), RawValue::Int(y)) => RawValue::Int(x * y),
            (RawValue::PosInt(x), RawValue::PosInt(y)) => RawValue::PosInt(x * y),
            _ => panic!("Cannot multiply mismatched RawValue types"),
        }
    }
}

impl Div for RawValue {
    type Output = RawValue;

    fn div(self, other: RawValue) -> RawValue {
        // TODO: Catch binary as type
        match (self, other) {
            (RawValue::Float(x), RawValue::Float(y)) => RawValue::Float(x / y),
            (RawValue::Int8(x), RawValue::Int8(y)) => RawValue::Int8(x / y),
            (RawValue::PosInt8(x), RawValue::PosInt8(y)) => RawValue::PosInt8(x / y),
            (RawValue::Int(x), RawValue::Int(y)) => RawValue::Int(x / y),
            (RawValue::PosInt(x), RawValue::PosInt(y)) => RawValue::PosInt(x / y),
            _ => panic!("Cannot multiply mismatched RawValue types"),
        }
    }
}

impl Neg for RawValue {
    type Output = RawValue;

    fn neg(self) -> RawValue {
        match self {
            RawValue::Int(x) => RawValue::Int(-x),
            RawValue::Float(x) => RawValue::Float(-x),
            RawValue::Int8(x) => RawValue::Int8(-x),
            _ => panic!("Cannot negate non-numeric types"),
        }
    }
}

impl Rem for RawValue {
    type Output = RawValue;

    fn rem(self, other: RawValue) -> RawValue {
        // TODO: Catch binary as type
        match (self, other) {
            (RawValue::Int(x), RawValue::Int(y)) => RawValue::Int(x % y),
            (RawValue::Int8(x), RawValue::Int8(y)) => RawValue::Int8(x % y),
            (RawValue::PosInt(x), RawValue::PosInt(y)) => RawValue::PosInt(x % y),
            _ => panic!("Cannot calculate remainder for non-integer types"),
        }
    }
}

impl PartialEq for RawValue {
    fn eq(&self, other: &RawValue) -> bool {
        match (self, other) {
            (RawValue::Binary(x), RawValue::Binary(y)) => x == y,
            (RawValue::Float(x), RawValue::Float(y)) => approx_eq!(f32, *x, *y, ulps = 2),
            (RawValue::Int8(x), RawValue::Int8(y)) => x == y,
            (RawValue::PosInt8(x), RawValue::PosInt8(y)) => x == y,
            (RawValue::Int(x), RawValue::Int(y)) => x == y,
            (RawValue::PosInt(x), RawValue::PosInt(y)) => x == y,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RawValue;

    #[test]
    fn test_rawvalue_add() {
        let a = RawValue::Int(5);
        let b = RawValue::Int(7);
        let c = a + b;
        assert_eq!(c, RawValue::Int(12));

        let a = RawValue::Float(3.14);
        let b = RawValue::Float(2.71);
        let c = a + b;
        assert_eq!(c, RawValue::Float(5.85));

        let a = RawValue::Binary(true);
        let b = RawValue::Binary(false);
        let c = a + b;
        assert_eq!(c, RawValue::Binary(true));
    }

    #[should_panic]
    #[test]
    fn test_rawvalue_add_mismatched() {
        let a = RawValue::Int(5);
        let b = RawValue::Float(7.0);
        let _ = a + b;
    }

    #[test]
    fn test_rawvalue_sub() {
        let a = RawValue::Int(5);
        let b = RawValue::Int(7);
        let c = a - b;
        assert_eq!(c, RawValue::Int(-2));

        let a = RawValue::Float(3.14);
        let b = RawValue::Float(2.71);
        let c = a - b;
        assert_eq!(c, RawValue::Float(0.43));
    }

    // TODO: add test for incompatible operations for variant

    #[should_panic]
    #[test]
    fn test_rawvalue_sub_mismatched() {
        let a = RawValue::Int(5);
        let b = RawValue::Float(7.0);
        let _ = a - b;
    }

    #[test]
    fn test_rawvalue_mul() {
        let a = RawValue::Int(5);
        let b = RawValue::Int(7);
        let c = a * b;
        assert_eq!(c, RawValue::Int(35));

        let a = RawValue::Float(3.14);
        let b = RawValue::Float(2.71);
        let c = a * b;
        assert_eq!(c, RawValue::Float(8.5094));
    }

    #[should_panic]
    #[test]
    fn test_rawvalue_mul_mismatched() {
        let a = RawValue::PosInt8(5);
        let b = RawValue::Float(7.0);
        let _ = a * b;
    }

    #[test]
    fn test_rawvalue_div() {
        let a = RawValue::Int(5);
        let b = RawValue::Int(7);
        let c = a / b;
        assert_eq!(c, RawValue::Int(0));

        let a = RawValue::Int(7);
        let b = RawValue::Int(5);
        let c = a / b;
        assert_eq!(c, RawValue::Int(1));

        let a = RawValue::Float(3.14);
        let b = RawValue::Float(2.71);
        let c = a / b;
        assert_eq!(c, RawValue::Float(3.14 / 2.71));
    }

    #[should_panic]
    #[test]
    fn test_rawvalue_div_mismatched() {
        let a = RawValue::Int(5);
        let b = RawValue::Float(7.0);
        let _ = a / b;
    }
}
