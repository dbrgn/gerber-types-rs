//! Custom data types used in the Gerber format.

use std::convert::{From, Into};
use std::num::FpCategory;
use std::i64;

use conv::TryFrom;

use ::GerberError;


/// The coordinate format specifies the number of integer and decimal places in
/// a coordinate number. For example, the `24` format specifies 2 integer and 4
/// decimal places. The number of decimal places must be 4, 5 or 6. The number
/// of integer places must be not more than 6. Thus the longest representable
/// coordinate number is `nnnnnn.nnnnnn`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoordinateFormat {
    pub integer: u8,
    pub decimal: u8,
}

impl CoordinateFormat {
    pub fn new(integer: u8, decimal: u8) -> Self {
        CoordinateFormat { integer: integer, decimal: decimal }
    }
}


/// Coordinate numbers are integers conforming to the rules set by the FS
/// command.
///
/// Coordinate numbers are integers. Explicit decimal points are not allowed.
///
/// A coordinate number must have at least one character. Zero therefore must
/// be encoded as `0`.
///
/// The value is stored as a 64 bit integer with 6 decimal places.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CoordinateNumber {
    nano: i64,
}

const DECIMAL_PLACES_CHARS: u8 = 6;
const DECIMAL_PLACES_FACTOR: i64 = 1_000_000;

impl TryFrom<f64> for CoordinateNumber {
    type Err = GerberError;
    fn try_from(val: f64) -> Result<Self, Self::Err> {
        match val.classify() {
            FpCategory::Nan => Err(GerberError::ConversionError("Value is NaN".into())),
            FpCategory::Infinite => Err(GerberError::ConversionError("Value is infinite".into())),
            FpCategory::Zero | FpCategory::Subnormal => Ok(CoordinateNumber { nano: 0 }),
            FpCategory::Normal => {
                let multiplied = val * DECIMAL_PLACES_FACTOR as f64;
                if (multiplied > i64::MAX as f64) || (multiplied < i64::MIN as f64) {
                    Err(GerberError::ConversionError("Value is out of bounds".into()))
                } else {
                    Ok(CoordinateNumber { nano: multiplied as i64 })
                }
            }
        }
    }
}

impl Into<f64> for CoordinateNumber {
    fn into(self) -> f64 {
        (self.nano as f64) / DECIMAL_PLACES_FACTOR as f64
    }
}

macro_rules! impl_from_integer {
    ($class:ty) => {
        impl From<$class> for CoordinateNumber {
            fn from(val: $class) -> Self {
                CoordinateNumber { nano: val as i64 * DECIMAL_PLACES_FACTOR }
            }
        }
    }
}

// These are the types we can safely multiply with DECIMAL_PLACES_FACTOR
// without the risk of an overflow.
impl_from_integer!(i8);
impl_from_integer!(i16);
impl_from_integer!(i32);
impl_from_integer!(u8);
impl_from_integer!(u16);

impl CoordinateNumber {
    pub fn gerber(&self, format: &CoordinateFormat) -> Result<String, GerberError> {
        // Format invariants
        if format.decimal > DECIMAL_PLACES_CHARS {
            return Err(GerberError::CoordinateFormatError("Invalid precision: Too high!".into()))
        }

        // If value is 0, return corresponding string
        if self.nano == 0 {
            return Ok("0".to_string());
        }

        // Get integer part by doing floor division
        let integer: i64 = self.nano / DECIMAL_PLACES_FACTOR;
        if integer > 10i64.pow(format.integer as u32) {
            return Err(GerberError::CoordinateFormatError("Decimal is too large for chosen format".into()));
        }

        // Get decimal part with proper rounding
        let divisor: i64 = 10i64.pow((DECIMAL_PLACES_CHARS - format.decimal) as u32);
        let decimal: i64 = ((self.nano % DECIMAL_PLACES_FACTOR).abs() + (divisor / 2)) / divisor;

        // Convert to string
        Ok(format!("{}{:0<width$}", integer, decimal, width=format.decimal as usize))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::f64;
    use conv::TryFrom;

    #[test]
    /// Test integer to coordinate number conversion
    fn test_from_i8() {
        let a = CoordinateNumber { nano: 13000000 };
        let b = CoordinateNumber::from(13i8);
        assert_eq!(a, b);

        let c = CoordinateNumber { nano: -99000000 };
        let d = CoordinateNumber::from(-99i8);
        assert_eq!(c, d);
    }

    #[test]
    /// Test integer to coordinate number conversion
    fn test_from_i32() {
        let a = CoordinateNumber { nano: 13000000 };
        let b = CoordinateNumber::from(13);
        assert_eq!(a, b);

        let c = CoordinateNumber { nano: -998000000 };
        let d = CoordinateNumber::from(-998);
        assert_eq!(c, d);
    }

    #[test]
    /// Test float to coordinate number conversion
    fn test_try_from_f64_success() {
        let a = CoordinateNumber { nano: 1375000i64 };
        let b = CoordinateNumber::try_from(1.375f64).unwrap();
        assert_eq!(a, b);

        let c = CoordinateNumber { nano: 123456888888i64 };
        let d = CoordinateNumber::try_from(123456.888888f64).unwrap();
        assert_eq!(c, d);

        let e = CoordinateNumber { nano: 0i64 };
        let f = CoordinateNumber::try_from(0f64).unwrap();
        assert_eq!(e, f);

        let g = CoordinateNumber { nano: -12345678 };
        let h = CoordinateNumber::try_from(-12.345678).unwrap();
        assert_eq!(g, h);
    }

    #[test]
    /// Test failing float to coordinate number conversion
    fn test_try_from_f64_fail() {
        let cn1 = CoordinateNumber::try_from(f64::NAN);
        assert!(cn1.is_err());

        let cn2 = CoordinateNumber::try_from(f64::INFINITY);
        assert!(cn2.is_err());

        let cn3 = CoordinateNumber::try_from(f64::MAX - 1.0);
        assert!(cn3.is_err());

        let cn4 = CoordinateNumber::try_from(f64::MIN + 1.0);
        assert!(cn4.is_err());
    }

    #[test]
    /// Test coordinate number to float conversion
    fn test_into_f64() {
        let a: f64 = CoordinateNumber { nano: 1375000i64 }.into();
        let b = 1.375f64;
        assert_eq!(a, b);

        let c: f64 = CoordinateNumber { nano: 123456888888i64 }.into();
        let d = 123456.888888f64;
        assert_eq!(c, d);

        let e: f64 = CoordinateNumber { nano: 0i64 }.into();
        let f = 0f64;
        assert_eq!(e, f);
    }

    #[test]
    /// Test coordinate number to string conversion when it's 0
    fn test_formatted_zero() {
        let cf1 = CoordinateFormat::new(6, 6);
        let cf2 = CoordinateFormat::new(2, 4);

        let a = CoordinateNumber { nano: 0 }.gerber(&cf1).unwrap();
        let b = CoordinateNumber { nano: 0 }.gerber(&cf2).unwrap();
        assert_eq!(a, "0".to_string());
        assert_eq!(b, "0".to_string());
    }

    #[test]
    /// Test coordinate number to string conversion when the decimal part is 0
    fn test_formatted_decimal_zero() {
        let cf1 = CoordinateFormat::new(6, 6);
        let cf2 = CoordinateFormat::new(2, 4);

        let a = CoordinateNumber { nano: 10000000 }.gerber(&cf1).unwrap();
        let b = CoordinateNumber { nano: 20000000 }.gerber(&cf2).unwrap();
        assert_eq!(a, "10000000".to_string());
        assert_eq!(b, "200000".to_string());
    }

    #[test]
    /// Test coordinate number to string conversion
    fn test_formatted_65() {
        let cf = CoordinateFormat::new(6, 5);
        let d = CoordinateNumber { nano: 123456789012 }.gerber(&cf).unwrap();
        assert_eq!(d, "12345678901".to_string());
    }

    #[test]
    /// Test coordinate number to string conversion
    fn test_formatted_54() {
        let cf = CoordinateFormat::new(5, 4);
        let d = CoordinateNumber { nano: 12345678901 }.gerber(&cf).unwrap();
        assert_eq!(d, "123456789".to_string());
    }

    #[test]
    /// Test coordinate number to string conversion failure
    fn test_formatted_number_too_large() {
        let cf = CoordinateFormat::new(4, 5);
        let d = CoordinateNumber { nano: 12345000000 }.gerber(&cf);
        assert!(d.is_err());
    }

    #[test]
    /// Test coordinate number to string conversion (rounding of decimal part)
    fn test_formatted_44_rounding() {
        let cf = CoordinateFormat::new(4, 4);
        let d = CoordinateNumber { nano: 1234432199 }.gerber(&cf).unwrap();
        assert_eq!(d, "12344322".to_string());
    }

    #[test]
    /// Test negative coordinate number to string conversion
    fn test_formatted_negative_rounding() {
        let cf = CoordinateFormat::new(6, 4);
        let d = CoordinateNumber { nano: -123456789099 }.gerber(&cf).unwrap();
        assert_eq!(d, "-1234567891".to_string());
    }

}
