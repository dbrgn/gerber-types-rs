/// All types that implement this trait can be converted to Gerber Code.
pub trait GerberCode {
    fn to_code(&self) -> String;
}

/// Automatically implement GerberCode trait for struct types
/// that are based on x and y attributes.
macro_rules! impl_xy_gerbercode {
    ($class:ty, $x:expr, $y: expr) => {
        impl GerberCode for $class {
            fn to_code(&self) -> String {
                let mut code = String::new();
                if let Some(x) = self.x {
                    code = format!("{}{}", $x, x);
                }
                if let Some(y) = self.y {
                    code.push_str(&format!("{}{}", $y, y));
                }
                code
            }
        }
    }
}

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

impl_xy_gerbercode!(Coordinates, "X", "Y");

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

impl_xy_gerbercode!(CoordinateOffset, "I", "J");

#[derive(Debug)]
pub enum Command {
    FunctionCode(FunctionCode),
    ExtendedCode(ExtendedCode),
}

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

#[derive(Debug)]
pub enum Operation {
    Interpolate(Coordinates, Option<CoordinateOffset>),
    Move(Coordinates),
    Flash(Coordinates),
}

impl GerberCode for Operation {
    fn to_code(&self) -> String {
        match *self {
            Operation::Interpolate(ref coords, ref offset) => format!("{}{}D01*", coords.to_code(), offset.to_code()),
            Operation::Move(ref coords) => format!("{}D02*", coords.to_code()),
            Operation::Flash(ref coords) => format!("{}D03*", coords.to_code()),
        }
    }
}

#[derive(Debug)]
pub enum DCode {
    Operation,
    SelectAperture,
}

#[derive(Debug)]
pub enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

impl GerberCode for InterpolationMode {
    fn to_code(&self) -> String {
        match *self {
            InterpolationMode::Linear => "G01*",
            InterpolationMode::ClockwiseCircular => "G02*",
            InterpolationMode::CounterclockwiseCircular => "G03*",
        }.to_string()
    }
}

#[derive(Debug)]
pub enum QuadrantMode {
    Single,
    Multi,
}

impl GerberCode for QuadrantMode {
    fn to_code(&self) -> String {
        match *self {
            QuadrantMode::Single => "G74*",
            QuadrantMode::Multi => "G75*",
        }.to_string()
    }
}

#[derive(Debug)]
pub enum GCode {
    InterpolationMode(InterpolationMode),
    RegionMode(bool),
    QuadrantMode(QuadrantMode),
    Comment(String),
}

impl GerberCode for GCode {
    fn to_code(&self) -> String {
        match *self {
            GCode::InterpolationMode(ref mode) => mode.to_code(),
            GCode::RegionMode(enabled) => if enabled { "G36*".to_string() } else { "G37*".to_string() },
            GCode::QuadrantMode(ref mode) => mode.to_code(),
            GCode::Comment(ref comment) => format!("G04 {} *", comment),
        }
    }
}

#[derive(Debug)]
pub enum MCode {
    EndOfFile,
}

impl GerberCode for MCode {
    fn to_code(&self) -> String {
        match *self {
            MCode::EndOfFile => "M02*",
        }.to_string()
    }
}

/// Implement GerberCode for Vectors of types that implement GerberCode.
impl<T: GerberCode> GerberCode for Vec<T> {
    fn to_code(&self) -> String {
        self.iter()
            .map(|g| g.to_code())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// Implement GerberCode for Option<T: GerberCode>
impl<T: GerberCode> GerberCode for Option<T> {
    fn to_code(&self) -> String {
        match *self {
            Some(ref v) => v.to_code(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Command, FunctionCode};
    use super::{GCode, InterpolationMode, QuadrantMode};
    use super::{MCode};
    use super::{Operation, Coordinates, CoordinateOffset};
    use super::GerberCode;

    #[test]
    fn test_debug() {
        //! The debug representation should work properly.
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
    }

    #[test]
    fn test_to_code() {
        //! The to_code method of the GerberCode trait should generate strings.
        let comment = GCode::Comment("testcomment".to_string());
        assert_eq!(comment.to_code(), "G04 testcomment *".to_string());
    }

    #[test]
    fn test_vec_to_code() {
        //! A `Vec<T: GerberCode>` should also implement `GerberCode`.
        let mut v = Vec::new();
        v.push(GCode::Comment("comment 1".to_string()));
        v.push(GCode::Comment("another one".to_string()));
        assert_eq!(v.to_code(), "G04 comment 1 *\nG04 another one *".to_string());
    }

    #[test]
    fn test_interpolation_mode() {
        let mut commands = Vec::new();
        let c1 = GCode::InterpolationMode(InterpolationMode::Linear);
        let c2 = GCode::InterpolationMode(InterpolationMode::ClockwiseCircular);
        let c3 = GCode::InterpolationMode(InterpolationMode::CounterclockwiseCircular);
        commands.push(c1);
        commands.push(c2);
        commands.push(c3);
        assert_eq!(commands.to_code(), "G01*\nG02*\nG03*".to_string());
    }

    #[test]
    fn test_region_mode() {
        let mut commands = Vec::new();
        commands.push(GCode::RegionMode(true));
        commands.push(GCode::RegionMode(false));
        assert_eq!(commands.to_code(), "G36*\nG37*".to_string());
    }

    #[test]
    fn test_quadrant_mode() {
        let mut commands = Vec::new();
        commands.push(GCode::QuadrantMode(QuadrantMode::Single));
        commands.push(GCode::QuadrantMode(QuadrantMode::Multi));
        assert_eq!(commands.to_code(), "G74*\nG75*".to_string());
    }

    #[test]
    fn test_end_of_file() {
        let c = MCode::EndOfFile;
        assert_eq!(c.to_code(), "M02*".to_string());
    }

    #[test]
    fn test_coordinates() {
        macro_rules! assert_coords {
            ($x:expr, $y:expr, $result:expr) => {{
                assert_eq!(Coordinates { x: $x, y: $y }.to_code(), $result.to_string());
            }}
        }
        assert_coords!(Some(10), Some(20), "X10Y20");
        assert_coords!(None, None, ""); // TODO should we catch this?
        assert_coords!(Some(10), None, "X10");
        assert_coords!(None, Some(20), "Y20");
        assert_coords!(Some(0), Some(-400), "X0Y-400");
    }

    #[test]
    fn test_offset() {
        macro_rules! assert_coords {
            ($x:expr, $y:expr, $result:expr) => {{
                assert_eq!(CoordinateOffset { x: $x, y: $y }.to_code(), $result.to_string());
            }}
        }
        assert_coords!(Some(10), Some(20), "I10J20");
        assert_coords!(None, None, ""); // TODO should we catch this?
        assert_coords!(Some(10), None, "I10");
        assert_coords!(None, Some(20), "J20");
        assert_coords!(Some(0), Some(-400), "I0J-400");
    }

    #[test]
    fn test_operation_interpolate() {
        let c1 = Operation::Interpolate(
            Coordinates::new(100, 200),
            Some(CoordinateOffset::new(5, 10))
        );
        assert_eq!(c1.to_code(), "X100Y200I5J10D01*".to_string());
        let c2 = Operation::Interpolate(
            Coordinates::at_y(-200),
            None
        );
        assert_eq!(c2.to_code(), "Y-200D01*".to_string());
        let c3 = Operation::Interpolate(
            Coordinates::at_x(1),
            Some(CoordinateOffset::at_y(2))
        );
        assert_eq!(c3.to_code(), "X1J2D01*".to_string());
    }

    #[test]
    fn test_operation_move() {
        let c = Operation::Move(Coordinates::new(23, 42));
        assert_eq!(c.to_code(), "X23Y42D02*".to_string());
    }

    #[test]
    fn test_operation_flash() {
        let c = Operation::Flash(Coordinates::new(23, 42));
        assert_eq!(c.to_code(), "X23Y42D03*".to_string());
    }

}
