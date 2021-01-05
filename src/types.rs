//! Types for Gerber code generation.
//!
//! All types are stateless, meaning that they contain all information in order
//! to render themselves. This means for example that each `Coordinates`
//! instance contains a reference to the coordinate format to be used.

use std::convert::From;

use crate::attributes;
use crate::coordinates;
use crate::extended_codes;
use crate::function_codes;
use crate::macros;

// Helper macros

macro_rules! impl_from {
    ($from:ty, $target:ty, $variant:expr) => {
        impl From<$from> for $target {
            fn from(val: $from) -> Self {
                $variant(val)
            }
        }
    };
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
    };
}

// Main categories

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCode {
    DCode(function_codes::DCode),
    GCode(function_codes::GCode),
    MCode(function_codes::MCode),
}

impl_from!(function_codes::DCode, FunctionCode, FunctionCode::DCode);
impl_from!(function_codes::GCode, FunctionCode, FunctionCode::GCode);
impl_from!(function_codes::MCode, FunctionCode, FunctionCode::MCode);

impl_command_fromfrom!(function_codes::DCode, FunctionCode::from);
impl_command_fromfrom!(function_codes::GCode, FunctionCode::from);
impl_command_fromfrom!(function_codes::MCode, FunctionCode::from);

#[derive(Debug, Clone, PartialEq)]
pub enum ExtendedCode {
    /// FS
    CoordinateFormat(coordinates::CoordinateFormat),
    /// MO
    Unit(extended_codes::Unit),
    /// AD
    ApertureDefinition(extended_codes::ApertureDefinition),
    /// AM
    ApertureMacro(macros::ApertureMacro),
    /// LP
    LoadPolarity(extended_codes::Polarity),
    /// SR
    StepAndRepeat(extended_codes::StepAndRepeat),
    /// TF
    FileAttribute(attributes::FileAttribute),
    /// TA
    ApertureAttribute(attributes::ApertureAttribute),
    /// TD
    DeleteAttribute(String),
}

impl_from!(
    coordinates::CoordinateFormat,
    ExtendedCode,
    ExtendedCode::CoordinateFormat
);
impl_from!(extended_codes::Unit, ExtendedCode, ExtendedCode::Unit);
impl_from!(
    extended_codes::ApertureDefinition,
    ExtendedCode,
    ExtendedCode::ApertureDefinition
);
impl_from!(
    macros::ApertureMacro,
    ExtendedCode,
    ExtendedCode::ApertureMacro
);
impl_from!(
    extended_codes::Polarity,
    ExtendedCode,
    ExtendedCode::LoadPolarity
);
impl_from!(
    extended_codes::StepAndRepeat,
    ExtendedCode,
    ExtendedCode::StepAndRepeat
);
impl_from!(
    attributes::FileAttribute,
    ExtendedCode,
    ExtendedCode::FileAttribute
);
impl_from!(
    attributes::ApertureAttribute,
    ExtendedCode,
    ExtendedCode::ApertureAttribute
);

impl_command_fromfrom!(coordinates::CoordinateFormat, ExtendedCode::from);
impl_command_fromfrom!(extended_codes::Unit, ExtendedCode::from);
impl_command_fromfrom!(extended_codes::ApertureDefinition, ExtendedCode::from);
impl_command_fromfrom!(macros::ApertureMacro, ExtendedCode::from);
impl_command_fromfrom!(extended_codes::Polarity, ExtendedCode::from);
impl_command_fromfrom!(extended_codes::StepAndRepeat, ExtendedCode::from);
impl_command_fromfrom!(attributes::FileAttribute, ExtendedCode::from);
impl_command_fromfrom!(attributes::ApertureAttribute, ExtendedCode::from);

#[cfg(test)]
mod test {
    use super::*;

    use std::io::BufWriter;

    use crate::extended_codes::Polarity;
    use crate::function_codes::GCode;
    use crate::traits::GerberCode;

    #[test]
    fn test_debug() {
        //! The debug representation should work properly.
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
    }

    #[test]
    fn test_function_code_serialize() {
        //! A `FunctionCode` should implement `GerberCode`
        let c = FunctionCode::GCode(GCode::Comment("comment".to_string()));
        assert_code!(c, "G04 comment *\n");
    }

    #[test]
    fn test_function_code_from_gcode() {
        let comment = GCode::Comment("hello".into());
        let f1: FunctionCode = FunctionCode::GCode(comment.clone());
        let f2: FunctionCode = comment.into();
        assert_eq!(f1, f2);
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
    fn test_extended_code_from_polarity() {
        let e1: ExtendedCode = ExtendedCode::LoadPolarity(Polarity::Dark);
        let e2: ExtendedCode = Polarity::Dark.into();
        assert_eq!(e1, e2);
    }
}
