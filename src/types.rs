//! Types for Gerber code generation.
//!
//! All types are stateless, meaning that they contain all information in order
//! to render themselves. This means for example that each `Coordinates`
//! instance contains a reference to the coordinate format to be used.

use std::convert::Into;

use ::{CoordinateFormat, CoordinateNumber};


/// Coordinates are part of an operation.
///
/// Coordinates are modal. If an X is omitted, the X coordinate of the
/// current point is used. Similar for Y.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinates {
    pub x: Option<CoordinateNumber>,
    pub y: Option<CoordinateNumber>,
    pub format: CoordinateFormat,
}

impl Coordinates {
    pub fn new<T, U>(x: T, y: U, format: CoordinateFormat) -> Self
            where T: Into<CoordinateNumber>, U: Into<CoordinateNumber> {
        Coordinates { x: Some(x.into()), y: Some(y.into()), format: format }
    }

    pub fn at_x<T>(x: T, format: CoordinateFormat) -> Self where T: Into<CoordinateNumber> {
        Coordinates { x: Some(x.into()), y: None, format: format }
    }

    pub fn at_y<T>(y: T, format: CoordinateFormat) -> Self where T: Into<CoordinateNumber> {
        Coordinates { x: None, y: Some(y.into()), format: format }
    }
}

/// Coordinate offsets can be used for interpolate operations in circular
/// interpolation mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinateOffset {
    pub x: Option<CoordinateNumber>,
    pub y: Option<CoordinateNumber>,
    pub format: CoordinateFormat,
}

impl CoordinateOffset {
    pub fn new<T, U>(x: T, y: U, format: CoordinateFormat) -> Self
            where T: Into<CoordinateNumber>, U: Into<CoordinateNumber> {
        CoordinateOffset { x: Some(x.into()), y: Some(y.into()), format: format }
    }

    pub fn at_x<T>(x: T, format: CoordinateFormat) -> Self where T: Into<CoordinateNumber> {
        CoordinateOffset { x: Some(x.into()), y: None, format: format }
    }

    pub fn at_y<T>(y: T, format: CoordinateFormat) -> Self where T: Into<CoordinateNumber> {
        CoordinateOffset { x: None, y: Some(y.into()), format: format }
    }
}


// Root type

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    FunctionCode(FunctionCode),
    ExtendedCode(ExtendedCode),
}


// Main categories

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCode {
    DCode(DCode),
    GCode(GCode),
    MCode(MCode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtendedCode {
    /// FS
    CoordinateFormat(CoordinateFormat),
    /// MO
    Unit(Unit),
    /// AD
    ApertureDefinition(ApertureDefinition),
    /// AM
    ApertureMacro(::macros::ApertureMacro),
    /// LP
    LoadPolarity(Polarity),
    /// SR
    StepAndRepeat(StepAndRepeat),
    /// TF
    FileAttribute(::attributes::FileAttribute),
    /// TA
    ApertureAttribute(::attributes::ApertureAttribute),
    /// TD
    DeleteAttribute(String),
}


// Function codes

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DCode {
    Operation(Operation),
    SelectAperture(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GCode {
    InterpolationMode(InterpolationMode),
    RegionMode(bool),
    QuadrantMode(QuadrantMode),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MCode {
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// D01 Command
    Interpolate(Coordinates, Option<CoordinateOffset>),
    /// D02 Command
    Move(Coordinates),
    /// D03 Command
    Flash(Coordinates),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuadrantMode {
    Single,
    Multi,
}


// Extended codes

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    Inches,
    Millimeters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApertureDefinition {
    pub code: i32,
    pub aperture: Aperture,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Aperture {
    Circle(Circle),
    Rectangle(Rectangular),
    Obround(Rectangular),
    Polygon(Polygon),
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub diameter: f64,
    pub hole_diameter: Option<f64>,
}

impl Circle {
    pub fn new(diameter: f64) -> Self {
        Circle {
            diameter: diameter,
            hole_diameter: None,
        }
    }

    pub fn with_hole(diameter: f64, hole_diameter: f64) -> Self {
        Circle {
            diameter: diameter,
            hole_diameter: Some(hole_diameter),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangular {
    pub x: f64,
    pub y: f64,
    pub hole_diameter: Option<f64>,
}

impl Rectangular {
    pub fn new(x: f64, y: f64) -> Self {
        Rectangular {
            x: x,
            y: y,
            hole_diameter: None,
        }
    }

    pub fn with_hole(x: f64, y: f64, hole_diameter: f64) -> Self {
        Rectangular {
            x: x,
            y: y,
            hole_diameter: Some(hole_diameter),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub diameter: f64,
    pub vertices: u8, // 3--12
    pub rotation: Option<f64>,
    pub hole_diameter: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Clear,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepAndRepeat {
    Open { repeat_x: u32, repeat_y: u32, distance_x: f64, distance_y: f64 },
    Close,
}


#[cfg(test)]
mod test {
    extern crate conv;

    use ::{CoordinateNumber, CoordinateFormat};
    use super::*;

    #[test]
    fn test_debug() {
        //! The debug representation should work properly.
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
    }

    #[test]
    fn test_coordinates_into() {
        let cf = CoordinateFormat::new(2, 4);
        let c1 = Coordinates::new(CoordinateNumber::from(1), CoordinateNumber::from(2), cf);
        let c2 = Coordinates::new(1, 2, cf);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_coordinates_into_mixed() {
        let cf = CoordinateFormat::new(2, 4);
        let c1 = Coordinates::new(CoordinateNumber::from(1), 2, cf);
        let c2 = Coordinates::new(1, 2, cf);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_circle_new() {
        let c1 = Circle::new(3.0);
        let c2 = Circle { diameter: 3.0, hole_diameter: None };
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_circle_with_hole() {
        let c1 = Circle::with_hole(3.0, 1.0);
        let c2 = Circle { diameter: 3.0, hole_diameter: Some(1.0) };
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_rectangular_new() {
        let r1 = Rectangular::new(2.0, 3.0);
        let r2 = Rectangular { x: 2.0, y: 3.0, hole_diameter: None };
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_rectangular_with_hole() {
        let r1 = Rectangular::with_hole(3.0, 2.0, 1.0);
        let r2 = Rectangular { x: 3.0, y: 2.0, hole_diameter: Some(1.0) };
        assert_eq!(r1, r2);
    }

}
