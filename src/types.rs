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
    FS,
    MO,
    AD,
    AM,
    SR,
    LP,
    TF,
    TA,
    TD,
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
