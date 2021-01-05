//! Function code types.

use std::io::Write;

use crate::errors::GerberResult;
use crate::traits::{GerberCode, PartialGerberCode};
use crate::coordinates::{Coordinates, CoordinateOffset};


// DCode

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DCode {
    Operation(Operation),
    SelectAperture(i32),
}

impl<W: Write> GerberCode<W> for DCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            DCode::Operation(ref operation) => operation.serialize(writer)?,
            DCode::SelectAperture(code) => write!(writer, "D{}*\n", code)?,
        };
        Ok(())
    }
}


// GCode

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GCode {
    InterpolationMode(InterpolationMode),
    RegionMode(bool),
    QuadrantMode(QuadrantMode),
    Comment(String),
}

impl<W: Write> GerberCode<W> for GCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            GCode::InterpolationMode(ref mode) => mode.serialize(writer)?,
            GCode::RegionMode(enabled) => if enabled {
                write!(writer, "G36*\n")?;
            } else {
                write!(writer, "G37*\n")?;
            },
            GCode::QuadrantMode(ref mode) => mode.serialize(writer)?,
            GCode::Comment(ref comment) => write!(writer, "G04 {} *\n", comment)?,
        };
        Ok(())
    }
}


// MCode

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MCode {
    EndOfFile,
}

impl<W: Write> GerberCode<W> for MCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            MCode::EndOfFile => write!(writer, "M02*\n")?,
        };
        Ok(())
    }
}


// Operation

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// D01 Command
    Interpolate(Coordinates, Option<CoordinateOffset>),
    /// D02 Command
    Move(Coordinates),
    /// D03 Command
    Flash(Coordinates),
}

impl<W: Write> GerberCode<W> for Operation {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Operation::Interpolate(ref coords, ref offset) => {
                coords.serialize_partial(writer)?;
                offset.serialize_partial(writer)?;
                write!(writer, "D01*\n")?;
            },
            Operation::Move(ref coords) => {
                coords.serialize_partial(writer)?;
                write!(writer, "D02*\n")?;
            },
            Operation::Flash(ref coords) => {
                coords.serialize_partial(writer)?;
                write!(writer, "D03*\n")?;
            }
        };
        Ok(())
    }
}


// InterpolationMode

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

impl<W: Write> GerberCode<W> for InterpolationMode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            InterpolationMode::Linear => write!(writer, "G01*\n")?,
            InterpolationMode::ClockwiseCircular => write!(writer, "G02*\n")?,
            InterpolationMode::CounterclockwiseCircular => write!(writer, "G03*\n")?,
        };
        Ok(())
    }
}


// QuadrantMode

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadrantMode {
    Single,
    Multi,
}

impl<W: Write> GerberCode<W> for QuadrantMode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            QuadrantMode::Single => write!(writer, "G74*\n")?,
            QuadrantMode::Multi => write!(writer, "G75*\n")?,
        };
        Ok(())
    }
}


#[cfg(test)]
mod test {
}
