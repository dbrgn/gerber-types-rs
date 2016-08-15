//! # Gerber commands
//!
//! This crate implements the basic building blocks of Gerber (RS-274X, aka
//! Extended Gerber version 2) code. It focusses on the low level types and does
//! not do any semantic checking.
//!
//! For example, you can use an aperture without defining it. This will
//! generate syntactically valid but semantially invalid Gerber code, but this
//! module won't complain.
//!
//! Minimal required Rust version: 1.6.

extern crate chrono;
extern crate uuid;

mod types;
mod attributes;
mod codegen;

pub use types::*;
pub use attributes::*;
pub use codegen::*;


#[cfg(test)]
mod test {
    use super::{GCode, InterpolationMode, QuadrantMode};
    use super::{MCode};
    use super::{DCode, Operation, Coordinates, CoordinateOffset};
    use super::GerberCode;

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

    #[test]
    fn test_select_aperture() {
        let c1 = DCode::SelectAperture(10);
        assert_eq!(c1.to_code(), "D10*".to_string());
        let c2 = DCode::SelectAperture(2147483647);
        assert_eq!(c2.to_code(), "D2147483647*".to_string());
    }

}
