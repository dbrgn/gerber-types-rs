/// Coordinates are part of an operation.
///
/// Coordinates are modal. If an X is omitted, the X coordinate of the
/// current point is used. Similar for Y.
#[derive(Debug)]
pub struct Coordinates {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates { x: Some(x), y: Some(y) }
    }

    pub fn at_x(x: i32) -> Self {
        Coordinates { x: Some(x), y: None }
    }

    pub fn at_y(y: i32) -> Self {
        Coordinates { x: None, y: Some(y) }
    }
}

/// Coordinate offsets can be used for interpolate operations in circular
/// interpolation mode.
#[derive(Debug)]
pub struct CoordinateOffset {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl CoordinateOffset {
    pub fn new(x: i32, y: i32) -> Self {
        CoordinateOffset { x: Some(x), y: Some(y) }
    }

    pub fn at_x(x: i32) -> Self {
        CoordinateOffset { x: Some(x), y: None }
    }

    pub fn at_y(y: i32) -> Self {
        CoordinateOffset { x: None, y: Some(y) }
    }
}


// Root type

#[derive(Debug)]
pub enum Command {
    FunctionCode(FunctionCode),
    ExtendedCode(ExtendedCode),
}


// Main categories

#[derive(Debug)]
pub enum FunctionCode {
    DCode(DCode),
    GCode(GCode),
    MCode(MCode),
}

#[derive(Debug)]
pub enum ExtendedCode {
    /// FS
    CoordinateFormat(u8, u8),
    /// MO
    Unit(Unit),
    /// AD
    ApertureDefinition(ApertureDefinition),
    /// AM
    ApertureMacro, // TODO
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

#[derive(Debug)]
pub enum DCode {
    Operation(Operation),
    SelectAperture(i32),
}

#[derive(Debug)]
pub enum GCode {
    InterpolationMode(InterpolationMode),
    RegionMode(bool),
    QuadrantMode(QuadrantMode),
    Comment(String),
}

#[derive(Debug)]
pub enum MCode {
    EndOfFile,
}

#[derive(Debug)]
pub enum Operation {
    Interpolate(Coordinates, Option<CoordinateOffset>),
    Move(Coordinates),
    Flash(Coordinates),
}

#[derive(Debug)]
pub enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

#[derive(Debug)]
pub enum QuadrantMode {
    Single,
    Multi,
}


// Extended codes

#[derive(Debug)]
pub enum Unit {
    Inches,
    Millimeters,
}

#[derive(Debug)]
pub struct ApertureDefinition {
    pub code: i32,
    pub aperture: Aperture,
}

#[derive(Debug)]
pub enum Aperture {
    Circle(Circle),
    Rectangle(Rectangular),
    Obround(Rectangular),
    Polygon(Polygon),
    Other(String),
}

#[derive(Debug)]
pub struct Circle {
    pub diameter: f64,
    pub hole_diameter: Option<f64>,
}

#[derive(Debug)]
pub struct Rectangular {
    pub x: f64,
    pub y: f64,
    pub hole_diameter: Option<f64>,
}

#[derive(Debug)]
pub struct Polygon {
    pub diameter: f64,
    pub vertices: u8, // 3--12
    pub rotation: Option<f64>,
    pub hole_diameter: Option<f64>,
}

#[derive(Debug)]
pub enum Polarity {
    Clear,
    Dark,
}

#[derive(Debug)]
pub enum StepAndRepeat {
    Open { repeat_x: u32, repeat_y: u32, distance_x: f64, distance_y: f64 },
    Close,
}


#[cfg(test)]
mod test {
    use super::{Command, FunctionCode, GCode};

    #[test]
    fn test_debug() {
        //! The debug representation should work properly.
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
    }

}
