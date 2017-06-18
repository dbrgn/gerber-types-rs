//! Types for Gerber code generation.
//!
//! All types are stateless, meaning that they contain all information in order
//! to render themselves. This means for example that each `Coordinates`
//! instance contains a reference to the coordinate format to be used.

use std::convert::From;

use coordinates::{CoordinateFormat, Coordinates, CoordinateOffset};


// Helper macros

macro_rules! impl_from {
    ($from:ty, $target:ty, $variant:expr) => {
        impl From<$from> for $target {
            fn from(val: $from) -> Self {
                $variant(val)
            }
        }
    }
}

// Root type

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    FunctionCode(FunctionCode),
    ExtendedCode(ExtendedCode),
}

impl_from!(FunctionCode, Command, Command::FunctionCode);
impl_from!(ExtendedCode, Command, Command::ExtendedCode);


macro_rules! impl_command_fromfrom {
    ($from:ty, $inner:path) => {
        impl From<$from> for Command {
            fn from(val: $from) -> Self {
                Command::from($inner(val))
            }
        }
    }
}


// Main categories

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCode {
    DCode(DCode),
    GCode(GCode),
    MCode(MCode),
}

impl_from!(DCode, FunctionCode, FunctionCode::DCode);
impl_from!(GCode, FunctionCode, FunctionCode::GCode);
impl_from!(MCode, FunctionCode, FunctionCode::MCode);

impl_command_fromfrom!(DCode, FunctionCode::from);
impl_command_fromfrom!(GCode, FunctionCode::from);
impl_command_fromfrom!(MCode, FunctionCode::from);


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

impl_from!(CoordinateFormat, ExtendedCode, ExtendedCode::CoordinateFormat);
impl_from!(Unit, ExtendedCode, ExtendedCode::Unit);
impl_from!(ApertureDefinition, ExtendedCode, ExtendedCode::ApertureDefinition);
impl_from!(::macros::ApertureMacro, ExtendedCode, ExtendedCode::ApertureMacro);
impl_from!(Polarity, ExtendedCode, ExtendedCode::LoadPolarity);
impl_from!(StepAndRepeat, ExtendedCode, ExtendedCode::StepAndRepeat);
impl_from!(::attributes::FileAttribute, ExtendedCode, ExtendedCode::FileAttribute);
impl_from!(::attributes::ApertureAttribute, ExtendedCode, ExtendedCode::ApertureAttribute);

impl_command_fromfrom!(CoordinateFormat, ExtendedCode::from);
impl_command_fromfrom!(Unit, ExtendedCode::from);
impl_command_fromfrom!(ApertureDefinition, ExtendedCode::from);
impl_command_fromfrom!(::macros::ApertureMacro, ExtendedCode::from);
impl_command_fromfrom!(Polarity, ExtendedCode::from);
impl_command_fromfrom!(StepAndRepeat, ExtendedCode::from);
impl_command_fromfrom!(::attributes::FileAttribute, ExtendedCode::from);
impl_command_fromfrom!(::attributes::ApertureAttribute, ExtendedCode::from);


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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadrantMode {
    Single,
    Multi,
}


// Extended codes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    use super::*;

    #[test]
    fn test_debug() {
        //! The debug representation should work properly.
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
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

    #[test]
    fn test_command_from_function_code() {
        let comment = FunctionCode::GCode(GCode::Comment("hello".into()));
        let c1: Command = Command::FunctionCode(comment.clone());
        let c2: Command = comment.into();
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_command_from_extended_code() {
        let delete_attr = ExtendedCode::DeleteAttribute("hello".into());
        let c1: Command = Command::ExtendedCode(delete_attr.clone());
        let c2: Command = delete_attr.into();
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_function_code_from_gcode() {
        let comment = GCode::Comment("hello".into());
        let f1: FunctionCode = FunctionCode::GCode(comment.clone());
        let f2: FunctionCode = comment.into();
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_extended_code_from_polarity() {
        let e1: ExtendedCode = ExtendedCode::LoadPolarity(Polarity::Dark);
        let e2: ExtendedCode = Polarity::Dark.into();
        assert_eq!(e1, e2);
    }

}
