use crate::errors::ErrorType;
use float_cmp::approx_eq;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Type used for passing between IO abstractions.
///
/// An enum is used to avoid a generic implementations of [`crate::io::IOEvent`]
/// and [`crate::io::Device`].
///
/// # Contribution
///
/// The implemented types have been chosen as a good fit for GPIO. However,
/// if a type is needed that is not here, feel free to initiate a pull request.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialOrd)]
pub enum Datum {
    Binary(bool),
    PosInt8(u8),
    Int8(i8),
    PosInt(u32),
    Int(i32),
    Float(f32),
}

impl Datum {
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Binary(_) => false,
            _ => true,
        }
    }
}

impl Default for Datum {
    fn default() -> Self {
        Datum::PosInt8(u8::default())
    }
}

impl Display for Datum {
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
impl TryFrom<u8> for Datum {
    type Error = ErrorType;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Datum::PosInt8(value))
    }
}
impl TryFrom<i8> for Datum {
    type Error = ErrorType;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Datum::Int8(value))
    }
}
impl TryFrom<u32> for Datum {
    type Error = ErrorType;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Datum::PosInt(value))
    }
}
impl TryFrom<i32> for Datum {
    type Error = ErrorType;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Datum::Int(value))
    }
}
impl TryFrom<f32> for Datum {
    type Error = ErrorType;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Datum::Float(value))
    }
}
impl TryFrom<bool> for Datum {
    type Error = ErrorType;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(Datum::Binary(value))
    }
}

// █▓▒░ Basic mathematical operations
impl Add for Datum {
    type Output = Datum;

    fn add(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Binary(x), Datum::Binary(y)) => Datum::Binary(x || y),
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(x + y),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(x + y),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(x + y),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(x + y),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(x + y),
            _ => panic!("Cannot add mismatched Datum types"),
        }
    }
}

impl Sub for Datum {
    type Output = Datum;

    fn sub(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(x - y),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(x - y),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(x - y),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(x - y),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(x - y),
            _ => panic!("Cannot subtract mismatched Datum types"),
        }
    }
}

impl Mul for Datum {
    type Output = Datum;

    fn mul(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(x * y),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(x * y),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(x * y),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(x * y),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(x * y),
            _ => panic!("Cannot multiply mismatched Datum types"),
        }
    }
}

impl Div for Datum {
    type Output = Datum;

    fn div(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(x / y),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(x / y),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(x / y),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(x / y),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(x / y),
            _ => panic!("Cannot multiply mismatched Datum types"),
        }
    }
}

impl Neg for Datum {
    type Output = Datum;

    fn neg(self) -> Datum {
        match self {
            Datum::Int(x) => Datum::Int(-x),
            Datum::Float(x) => Datum::Float(-x),
            Datum::Int8(x) => Datum::Int8(-x),
            Datum::Binary(x) => Datum::Binary(
                match x {
                    true => false,
                    false => true
                }
            ),
            _ => panic!("Cannot negate unsigned types"),
        }
    }
}

impl Rem for Datum {
    type Output = Datum;

    fn rem(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(x % y),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(x % y),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(x % y),
            _ => panic!("Cannot calculate remainder for non-integer types"),
        }
    }
}

impl PartialEq for Datum {
    fn eq(&self, other: &Datum) -> bool {
        match (self, other) {
            (Datum::Binary(x), Datum::Binary(y)) => x == y,
            (Datum::Float(x), Datum::Float(y)) => approx_eq!(f32, *x, *y, ulps = 2),
            (Datum::Int8(x), Datum::Int8(y)) => x == y,
            (Datum::PosInt8(x), Datum::PosInt8(y)) => x == y,
            (Datum::Int(x), Datum::Int(y)) => x == y,
            (Datum::PosInt(x), Datum::PosInt(y)) => x == y,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Datum;

    #[test]
    fn test_datum_add() {
        let a = Datum::Int(5);
        let b = Datum::Int(7);
        let c = a + b;
        assert_eq!(c, Datum::Int(12));

        let a = Datum::Float(3.14);
        let b = Datum::Float(2.71);
        let c = a + b;
        assert_eq!(c, Datum::Float(5.85));

        let a = Datum::Binary(true);
        let b = Datum::Binary(false);
        let c = a + b;
        assert_eq!(c, Datum::Binary(true));
    }

    #[should_panic]
    #[test]
    fn test_datum_add_mismatched() {
        let a = Datum::Int(5);
        let b = Datum::Float(7.0);
        let _ = a + b;
    }

    #[test]
    fn test_datum_sub() {
        let a = Datum::Int(5);
        let b = Datum::Int(7);
        let c = a - b;
        assert_eq!(c, Datum::Int(-2));

        let a = Datum::Float(3.14);
        let b = Datum::Float(2.71);
        let c = a - b;
        assert_eq!(c, Datum::Float(0.43));
    }

    // TODO: add test for incompatible operations for variant

    #[should_panic]
    #[test]
    fn test_datum_sub_mismatched() {
        let a = Datum::Int(5);
        let b = Datum::Float(7.0);
        let _ = a - b;
    }

    #[test]
    fn test_datum_mul() {
        let a = Datum::Int(5);
        let b = Datum::Int(7);
        let c = a * b;
        assert_eq!(c, Datum::Int(35));

        let a = Datum::Float(3.14);
        let b = Datum::Float(2.71);
        let c = a * b;
        assert_eq!(c, Datum::Float(8.5094));
    }

    #[should_panic]
    #[test]
    fn test_datum_mul_mismatched() {
        let a = Datum::PosInt8(5);
        let b = Datum::Float(7.0);
        let _ = a * b;
    }

    #[test]
    fn test_datum_div() {
        let a = Datum::Int(5);
        let b = Datum::Int(7);
        let c = a / b;
        assert_eq!(c, Datum::Int(0));

        let a = Datum::Int(7);
        let b = Datum::Int(5);
        let c = a / b;
        assert_eq!(c, Datum::Int(1));

        let a = Datum::Float(3.14);
        let b = Datum::Float(2.71);
        let c = a / b;
        assert_eq!(c, Datum::Float(3.14 / 2.71));
    }

    #[should_panic]
    #[test]
    fn test_datum_div_mismatched() {
        let a = Datum::Int(5);
        let b = Datum::Float(7.0);
        let _ = a / b;
    }
}
