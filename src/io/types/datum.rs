use crate::errors::ErrorType;
use float_cmp::approx_eq;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

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
    Binary(Option<bool>),
    PosInt8(Option<u8>),
    Int8(Option<i8>),
    PosInt(Option<u32>),
    Int(Option<i32>),
    Float(Option<f32>),
}

impl Datum {
    /// Helper function which returns `true` if [`Datum`] contains `Some`
    ///
    /// # Returns
    ///
    /// `true` if inner value is `Some`, otherwise returns `false`
    pub fn is_some(&self) -> bool {
        match self {
            Self::Binary(inner) => inner.is_some(),
            Self::PosInt8(inner) => inner.is_some(),
            Self::Int8(inner) => inner.is_some(),
            Self::PosInt(inner) => inner.is_some(),
            Self::Int(inner) => inner.is_some(),
            Self::Float(inner) => inner.is_some(),
        }
    }

    /// Helper function which returns `true` if [`Datum`] contains `None`
    ///
    /// # Returns
    ///
    /// `true` if inner value is `None`, otherwise returns `false`
    pub fn is_none(&self) -> bool {
        match self {
            Self::Binary(inner) => inner.is_none(),
            Self::PosInt8(inner) => inner.is_none(),
            Self::Int8(inner) => inner.is_none(),
            Self::PosInt(inner) => inner.is_none(),
            Self::Int(inner) => inner.is_none(),
            Self::Float(inner) => inner.is_none(),
        }
    }

    /// Helper function to determine if [`Datum`] variant is numeric or not
    ///
    /// # Returns
    ///
    /// `true` if internal variant is [`Datum::PosInt8`], [`Datum::Int8`],
    /// [`Datum::PosInt`], [`Datum::Int`], or [`Datum::Float`]
    ///
    /// Returns `false` if [`Datum::Binary`].
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Binary(_) => false,
            _ => true,
        }
    }

    /// Helper constructing `Binary` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::binary(true);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::binary(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `val`: `bool` to wrap in `Binary` variant. `bool` does not have to be wrapped in `Option`
    pub fn binary<T: Into<Option<bool>>>(val: T) -> Self {
        Self::Binary(val.into())
    }

    /// Helper constructing `PosInt8` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::uint8(3);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::uint8(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `num`: value to wrap in `PosInt8` variant. `u8` does not have to be wrapped in `Option`
    pub fn uint8<T: Into<Option<u8>>>(num: T) -> Self {
        Self::PosInt8(num.into())
    }

    /// Helper constructing `Int8` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::int8(-3);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::int8(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `num`: value to wrap in `Int8` variant. `i8` does not have to be wrapped in `Option`
    pub fn int8<T: Into<Option<i8>>>(num: T) -> Self {
        Self::Int8(num.into())
    }

    /// Helper constructing `PosInt` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::uint(1000);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::uint(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `num`: value to wrap in `PosInt` variant. `u32` does not have to be wrapped in `Option`
    pub fn uint<T: Into<Option<u32>>>(num: T) -> Self {
        Self::PosInt(num.into())
    }

    /// Helper constructing `Int` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::int(-3000);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::int(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `num`: value to wrap in float variant. `i32` does not have to be wrapped in `Option`
    pub fn int<T: Into<Option<i32>>>(num: T) -> Self {
        Self::Int(num.into())
    }

    /// Helper constructing `Float` variants
    ///
    /// # Examples
    ///
    /// ```
    /// use sensd::io::Datum;
    ///
    /// let datum = Datum::float(3.14);
    /// assert!(datum.is_some());
    ///
    /// let datum = Datum::float(None);
    /// assert!(datum.is_none());
    /// ```
    ///
    /// # Parameters
    ///
    /// - `num`: value to wrap in float variant. `f32` does not have to be wrapped in `Option`
    pub fn float<T: Into<Option<f32>>>(num: T) -> Self {
        Self::Float(num.into())
    }
}

impl Default for Datum {
    fn default() -> Self {
        Datum::PosInt8(None)
    }
}

impl Display for Datum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const NONE: &'static str = "none";
        write!(
            f,
            "{}",
            match self {
                Self::Binary(val) => {
                    if let Some(inner) = val {
                        if *inner {
                            "true"
                        } else {
                            "false"
                        }
                    } else {
                        NONE
                    }
                    .to_string()
                }
                Self::PosInt8(val) => {
                    if let Some(inner) = val {
                        inner.to_string()
                    } else {
                        NONE.to_string()
                    }
                }
                Self::Int8(val) => {
                    if let Some(inner) = val {
                        inner.to_string()
                    } else {
                        NONE.to_string()
                    }
                }
                Self::PosInt(val) => {
                    if let Some(inner) = val {
                        inner.to_string()
                    } else {
                        NONE.to_string()
                    }
                }
                Self::Int(val) => {
                    if let Some(inner) = val {
                        inner.to_string()
                    } else {
                        NONE.to_string()
                    }
                }
                Self::Float(val) => {
                    if let Some(inner) = val {
                        inner.to_string()
                    } else {
                        NONE.to_string()
                    }
                }
            }
        )
    }
}

/// Conversion from primitive types
impl TryFrom<u8> for Datum {
    type Error = ErrorType;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Datum::PosInt8(value.into()))
    }
}
impl TryFrom<i8> for Datum {
    type Error = ErrorType;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Datum::Int8(value.into()))
    }
}
impl TryFrom<u32> for Datum {
    type Error = ErrorType;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Datum::PosInt(value.into()))
    }
}
impl TryFrom<i32> for Datum {
    type Error = ErrorType;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Datum::Int(value.into()))
    }
}
impl TryFrom<f32> for Datum {
    type Error = ErrorType;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Datum::Float(value.into()))
    }
}
impl TryFrom<bool> for Datum {
    type Error = ErrorType;
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Ok(Datum::Binary(value.into()))
    }
}

// █▓▒░ Basic mathematical operations
impl Add for Datum {
    type Output = Datum;

    fn add(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Binary(x), Datum::Binary(y)) => Datum::Binary(if let Some(x) = x {
                if let Some(y) = y {
                    Some(x || y)
                } else {
                    None
                }
            } else {
                None
            }),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(add_inner(x, y)),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(add_inner(x, y)),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(add_inner(x, y)),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(add_inner(x, y)),
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(add_inner(x, y)),
            _ => panic!("Cannot add mismatched Datum types"),
        }
    }
}

impl Sub for Datum {
    type Output = Datum;

    fn sub(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(sub_inner(x, y)),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(sub_inner(x, y)),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(sub_inner(x, y)),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(sub_inner(x, y)),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(sub_inner(x, y)),
            _ => panic!("Cannot subtract mismatched Datum types"),
        }
    }
}

impl Mul for Datum {
    type Output = Datum;

    fn mul(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(mul_inner(x, y)),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(mul_inner(x, y)),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(mul_inner(x, y)),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(mul_inner(x, y)),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(mul_inner(x, y)),
            _ => panic!("Cannot multiply mismatched Datum types"),
        }
    }
}

impl Div for Datum {
    type Output = Datum;

    fn div(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Float(x), Datum::Float(y)) => Datum::Float(div_inner(x, y)),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(div_inner(x, y)),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => Datum::PosInt8(div_inner(x, y)),
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(div_inner(x, y)),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(div_inner(x, y)),
            _ => panic!("Cannot multiply mismatched Datum types"),
        }
    }
}

impl Neg for Datum {
    type Output = Datum;

    fn neg(self) -> Datum {
        match self {
            Datum::Int(x) => Datum::Int(neg_inner(x)),
            Datum::Float(x) => Datum::Float(neg_inner(x)),
            Datum::Int8(x) => Datum::Int8(neg_inner(x)),
            Datum::Binary(x) => Datum::Binary(if let Some(inner) = x {
                Some(inner.not())
            } else {
                None
            }),
            _ => panic!("Cannot negate unsigned types"),
        }
    }
}

impl Rem for Datum {
    type Output = Datum;

    fn rem(self, other: Datum) -> Datum {
        // TODO: Catch binary as type
        match (self, other) {
            (Datum::Int(x), Datum::Int(y)) => Datum::Int(rem_inner(x, y)),
            (Datum::Int8(x), Datum::Int8(y)) => Datum::Int8(rem_inner(x, y)),
            (Datum::PosInt(x), Datum::PosInt(y)) => Datum::PosInt(rem_inner(x, y)),
            _ => panic!("Cannot calculate remainder for non-integer types"),
        }
    }
}

impl PartialEq for Datum {
    /// Check to see if two types are equal.
    ///
    /// # Returns
    /// `true` if both values are the same variant and their inner values are equal.
    /// If both variants are the same, but both contain `None`, then `true` is returned
    /// Otherwise, `false` is returned. Note that `false` is returned for differing variants.
    fn eq(&self, other: &Datum) -> bool {
        match (self, other) {
            (Datum::Binary(x), Datum::Binary(y)) => eq_inner(x, y),
            (Datum::Float(x), Datum::Float(y)) => {
                if let Some(x) = x {
                    if let Some(y) = y {
                        return approx_eq!(f32, *x, *y, ulps = 2);
                    }
                }
                false
            }
            (Datum::Int8(x), Datum::Int8(y)) => eq_inner(x, y),
            (Datum::PosInt8(x), Datum::PosInt8(y)) => eq_inner(x, y),
            (Datum::Int(x), Datum::Int(y)) => eq_inner(x, y),
            (Datum::PosInt(x), Datum::PosInt(y)) => eq_inner(x, y),
            _ => false,
        }
    }
}

#[inline]
/// Add two optional values of the same type
///
/// # Returns
/// Sum of values if both values are `Some`; otherwise returns `None`
fn add_inner<T: Add + Add<Output = T>>(l: Option<T>, r: Option<T>) -> Option<T> {
    Some(l? + r?)
}

#[inline]
/// Subtract two optional values of the same type
///
/// # Returns
/// Difference between two values if both values are `Some`; otherwise returns `None`
fn sub_inner<T: Sub + Sub<Output = T>>(l: Option<T>, r: Option<T>) -> Option<T> {
    Some(l? - r?)
}

#[inline]
/// Multiply two optional values of the same type
///
/// # Returns
/// Product of two values if both values are `Some`; otherwise returns `None`
fn mul_inner<T: Mul + Mul<Output = T>>(l: Option<T>, r: Option<T>) -> Option<T> {
    Some(l? * r?)
}

#[inline]
/// Divide two optional values of the same type
///
/// # Returns
/// Product of two values if both values are `Some`; otherwise returns `None`
fn div_inner<T: Div + Div<Output = T>>(l: Option<T>, r: Option<T>) -> Option<T> {
    Some(l? / r?)
}

#[inline]
/// Negate the inner value
///
/// # Returns
/// `Some` with inverse of inner value if `x` is `Some`; otherwise returns `None`
fn neg_inner<T: Neg + Neg<Output = T>>(x: Option<T>) -> Option<T> {
    Some(x?.neg())
}

#[inline]
fn rem_inner<T: Rem + Rem<Output = T>>(l: Option<T>, r: Option<T>) -> Option<T> {
    Some(l? % r?)
}

#[inline]
/// Check if two optional values are equal
///
/// # Returns
/// If parameters are both `Some`, then the equality of their inner values is returned.
/// If both of the parameters are `None`, then `true` is returned.
/// Otherwise, if either parameter is `None`, then `true` is returned.
fn eq_inner<T: PartialEq>(l: &Option<T>, r: &Option<T>) -> bool {
    if l.is_none() && r.is_none() {
        return true;
    } else if let Some(l) = l {
        if let Some(r) = r {
            return l == r;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::io::types::datum::{add_inner, div_inner, mul_inner, sub_inner};
    use crate::io::Datum;

    #[test]
    fn test_add_inner() {
        let x = Some(15);
        let y = Some(1);

        assert_eq!(add_inner(x, y), Some(16));
        assert_eq!(add_inner(x, None), None);
        assert_eq!(add_inner(None, y), None);
        assert_eq!(add_inner::<u32>(None, None), None);
    }

    #[test]
    fn test_datum_add() {
        let a = Datum::Int(5.into());
        let b = Datum::Int(7.into());
        let c = a + b;
        assert_eq!(c, Datum::Int(12.into()));

        let a = Datum::Float(3.14.into());
        let b = Datum::Float(2.71.into());
        let c = a + b;
        assert_eq!(c, Datum::Float(5.85.into()));

        let a = Datum::Binary(true.into());
        let b = Datum::Binary(false.into());
        let c = a + b;
        assert_eq!(c, Datum::Binary(true.into()));
    }

    #[should_panic]
    #[test]
    fn test_datum_add_mismatched() {
        let a = Datum::Int(5.into());
        let b = Datum::Float(7.0.into());
        let _ = a + b;
    }

    #[test]
    fn test_sub_inner() {
        let x = Some(15);
        let y = Some(1);

        assert_eq!(sub_inner(x, y), Some(14));
        assert_eq!(sub_inner(x, None), None);
        assert_eq!(sub_inner(None, y), None);
        assert_eq!(sub_inner::<u32>(None, None), None);
    }

    #[test]
    fn test_datum_sub() {
        let a = Datum::int(5);
        let b = Datum::int(7);
        let c = a - b;
        assert_eq!(c, Datum::int(-2));

        let a = Datum::float(3.14);
        let b = Datum::float(2.71);
        let c = a - b;
        assert_eq!(c, Datum::float(0.43));
    }

    // TODO: add test for incompatible operations for variant

    #[should_panic]
    #[test]
    fn test_datum_sub_mismatched() {
        let a = Datum::int(5);
        let b = Datum::float(7.0);
        let _ = a - b;
    }

    #[test]
    fn test_mul_inner() {
        let x = Some(15);
        let y = Some(3);

        assert_eq!(mul_inner(x, y), Some(45));
        assert_eq!(mul_inner(x, None), None);
        assert_eq!(mul_inner(None, y), None);
        assert_eq!(mul_inner::<u8>(None, None), None);
    }

    #[test]
    fn test_datum_mul() {
        let a = Datum::int(5);
        let b = Datum::int(7);
        let c = a * b;
        assert_eq!(c, Datum::int(35));

        let a = Datum::float(3.14);
        let b = Datum::float(2.71);
        let c = a * b;
        assert_eq!(c, Datum::float(8.5094));
    }

    #[should_panic]
    #[test]
    fn test_datum_mul_mismatched() {
        let a = Datum::uint8(5);
        let b = Datum::float(7.0);
        let _ = a * b;
    }
    #[test]
    fn test_div_inner() {
        let x = Some(15);
        let y = Some(3);

        assert_eq!(div_inner(x, y), Some(5));
        assert_eq!(div_inner(x, None), None);
        assert_eq!(div_inner(None, y), None);
        assert_eq!(div_inner::<u32>(None, None), None);
    }

    #[test]
    fn test_datum_div() {
        let a = Datum::int(5);
        let b = Datum::int(7);
        let c = a / b;
        assert_eq!(c, Datum::int(0));

        let a = Datum::int(7);
        let b = Datum::int(5);
        let c = a / b;
        assert_eq!(c, Datum::int(1));

        let a = Datum::float(3.14);
        let b = Datum::float(2.71);
        let c = a / b;
        assert_eq!(c, Datum::float(3.14 / 2.71));
    }

    #[should_panic]
    #[test]
    fn test_datum_div_mismatched() {
        let a = Datum::int(5);
        let b = Datum::float(7.0);
        let _ = a / b;
    }
}
