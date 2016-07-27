/// All types that implement this trait can be converted to Gerber Code.
pub trait GerberCode {
    fn to_code(&self) -> String;
}

/// Automatically implement GerberCode trait for struct types
/// that are based on x and y attributes.
macro_rules! impl_xy_gerbercode {
    ($class:ty) => {
        impl GerberCode for $class {
            fn to_code(&self) -> String {
                let mut code = String::new();
                if let Some(x) = self.x {
                    code = format!("X{}", x);
                }
                if let Some(y) = self.y {
                    code.push_str(&format!("Y{}", y));
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

impl_xy_gerbercode!(Coordinates);

/// Coordinate offsets can be used for interpolate operations in circular
/// interpolation mode.
#[derive(Debug)]
pub struct CoordinateOffset {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl_xy_gerbercode!(CoordinateOffset);

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

#[cfg(test)]
mod test {
    use super::{Command, FunctionCode};
    use super::{GCode, InterpolationMode, QuadrantMode};
    use super::{MCode};
    use super::{DCode, Coordinates, CoordinateOffset};
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
        assert_coords!(Some(10), Some(20), "X10Y20");
        assert_coords!(None, None, ""); // TODO should we catch this?
        assert_coords!(Some(10), None, "X10");
        assert_coords!(None, Some(20), "Y20");
        assert_coords!(Some(0), Some(-400), "X0Y-400");
    }

}
