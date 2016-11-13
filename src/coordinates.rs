//! Custom data types used in the Gerber format.

use std::convert::{From, Into};


/// The coordinate format specifies the number of integer and decimal places in
/// a coordinate number. For example, the `24` format specifies 2 integer and 4
/// decimal places. The number of decimal places must be 4, 5 or 6. The number
/// of integer places must be not more than 6. Thus the longest representable
/// coordinate number is `nnnnnn.nnnnnn`. The same format must be defined for X
/// and Y. Signs in coordinates are allowed; the `+` sign is optional.
#[derive(Debug, Copy, Clone)]
pub struct CoordinateFormat(pub u8, pub u8);


/// Decimals are a sequence of one or more digits with an optional decimal
/// point optionally preceded by a ‘+’ or a ‘-’ sign. They must fit in an IEEE
/// double.
///
/// The value is stored as a 64 bit integer with 9 decimal places.
#[derive(Debug, PartialEq)]
pub struct Decimal(i64);

const DECIMAL_PLACES: i64 = 1_000_000_000;

impl From<f64> for Decimal {
    fn from(val: f64) -> Decimal {
        let integer = (val.trunc() as i64) * DECIMAL_PLACES;
        let decimal = ((val - val.trunc()) * DECIMAL_PLACES as f64).trunc() as i64;
        Decimal(integer + decimal)
    }
}

impl Into<f64> for Decimal {
    fn into(self) -> f64 {
        (self.0 as f64) / DECIMAL_PLACES as f64
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test float to decimal conversion
    fn test_from_f64() {

        let a = Decimal(1375000000i64);
        let b = Decimal::from(1.375f64);
        assert_eq!(a, b);

        let c = Decimal(123456888888000i64);
        let d = Decimal::from(123456.888888f64);
        assert_eq!(c, d);
    }

    #[test]
    /// Test decimal to float conversion
    fn test_into_f64() {

        let a: f64 = Decimal(1375000000i64).into();
        let b = 1.375f64;
        assert_eq!(a, b);

        let c: f64 = Decimal(123456888888000i64).into();
        let d = 123456.888888f64;
        assert_eq!(c, d);
    }

}
