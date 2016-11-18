//! Custom data types used in the Gerber format.

use std::convert::{From, Into};

use ::GerberError;


/// The coordinate format specifies the number of integer and decimal places in
/// a coordinate number. For example, the `24` format specifies 2 integer and 4
/// decimal places. The number of decimal places must be 4, 5 or 6. The number
/// of integer places must be not more than 6. Thus the longest representable
/// coordinate number is `nnnnnn.nnnnnn`.
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CoordinateNumber {
    nano: i64,
}

const DECIMAL_PLACES_CHARS: u8 = 6;
const DECIMAL_PLACES: i64 = 1_000_000;

impl From<f64> for CoordinateNumber {
    fn from(val: f64) -> CoordinateNumber {
        CoordinateNumber { nano: (val * DECIMAL_PLACES as f64) as i64 }
    }
}

impl Into<f64> for CoordinateNumber {
    fn into(self) -> f64 {
        (self.nano as f64) / DECIMAL_PLACES as f64
    }
}

impl CoordinateNumber {
    fn gerber(&self, format: &CoordinateFormat) -> Result<String, GerberError> {
        // Format invariants
        if format.decimal > DECIMAL_PLACES_CHARS {
            return Err(GerberError::CoordinateFormatError("Invalid precision: Too high!".into()))
        }

        // If value is 0, return corresponding string
        if self.nano == 0 {
            return Ok("0".to_string());
        }

        // Convert to string
        let integer: i64 = self.nano / DECIMAL_PLACES;
        if integer > 10i64.pow(format.integer as u32) {
            return Err(GerberError::CoordinateFormatError("Decimal is too large for chosen format".into()));
        }
        let divisor: i64 = 10i64.pow((DECIMAL_PLACES_CHARS - format.decimal) as u32);
        let decimal: i64 = (self.nano % DECIMAL_PLACES) / divisor;
        Ok(format!("{}{}", integer, decimal))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test float to decimal conversion
    fn test_from_f64() {

        let a = CoordinateNumber { nano: 1375000i64 };
        let b = CoordinateNumber::from(1.375f64);
        assert_eq!(a, b);

        let c = CoordinateNumber { nano: 123456888888i64 };
        let d = CoordinateNumber::from(123456.888888f64);
        assert_eq!(c, d);

        let e = CoordinateNumber { nano: 0i64 };
        let f = CoordinateNumber::from(0f64);
        assert_eq!(e, f);
    }

    #[test]
    /// Test decimal to float conversion
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
    /// Test decimal to string conversion when it's 0
    fn test_formatted_zero() {
        let cf1 = CoordinateFormat::new(6, 6);
        let cf2 = CoordinateFormat::new(2, 4);

        let a = CoordinateNumber { nano: 0 }.gerber(&cf1).unwrap();
        let b = CoordinateNumber { nano: 0 }.gerber(&cf2).unwrap();
        assert_eq!(a, "0".to_string());
        assert_eq!(b, "0".to_string());
    }

    #[test]
    /// Test decimal to string conversion
    fn test_formatted_66() {
        let cf = CoordinateFormat::new(6, 5);
        let d = CoordinateNumber { nano: 123456789012 }.gerber(&cf).unwrap();
        assert_eq!(d, "12345678901".to_string());
    }

    #[test]
    /// Test decimal to string conversion
    fn test_formatted_54() {
        let cf = CoordinateFormat::new(5, 4);
        let d = CoordinateNumber { nano: 12345678901 }.gerber(&cf).unwrap();
        assert_eq!(d, "123456789".to_string());
    }

    #[test]
    /// Test decimal to string conversion failure
    fn test_formatted_number_too_large() {
        let cf = CoordinateFormat::new(4, 5);
        let d = CoordinateNumber { nano: 12345000000 }.gerber(&cf);
        assert!(d.is_err());
    }

    #[test]
    /// Test decimal to string conversion (rounding of decimal part)
    fn test_formatted_44_round_decimal() {
        let cf = CoordinateFormat::new(4, 4);
        let d = CoordinateNumber { nano: 1234432199 }.gerber(&cf).unwrap();
        assert_eq!(d, "12344322".to_string());
    }

}
