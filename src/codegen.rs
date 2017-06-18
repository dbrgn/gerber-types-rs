use std::io::Write;

use types::*;
use attributes::*;
use ::GerberResult;

/// All types that implement this trait can be converted to a complete Gerber
/// Code line. Generated code should end with a newline.
pub trait GerberCode<W: Write> {
    fn serialize(&self, writer: &mut W) -> GerberResult<()>;
}

/// All types that implement this trait can be converted to a Gerber Code
/// representation.
pub trait PartialGerberCode<W: Write> {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()>;
}

/// Implement `PartialGerberCode` for booleans
impl<W: Write> PartialGerberCode<W> for bool {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        if *self {
            write!(writer, "1")?;
        } else {
            write!(writer, "0")?;
        };
        Ok(())
    }
}

/// Implement `GerberCode` for Vectors of types that are `GerberCode`.
impl<W: Write, G: GerberCode<W>> GerberCode<W> for Vec<G> {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
}

/// Implement `PartialGerberCode` for `Option<T: PartialGerberCode>`
impl<T: PartialGerberCode<W>, W: Write> PartialGerberCode<W> for Option<T> {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        if let Some(ref val) = *self {
            val.serialize_partial(writer)?;
        }
        Ok(())
    }
}

/// Automatically implement `PartialGerberCode` trait for struct types
/// that are based on `x` and `y` attributes.
macro_rules! impl_xy_partial_gerbercode {
    ($class:ty, $x:expr, $y: expr) => {
        impl<W: Write> PartialGerberCode<W> for $class {
            fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
                if let Some(x) = self.x {
                    write!(writer, "{}{}", $x, x.gerber(&self.format)?)?;
                }
                if let Some(y) = self.y {
                    write!(writer, "{}{}", $y, y.gerber(&self.format)?)?;
                }
                Ok(())
            }
        }
    }
}

impl_xy_partial_gerbercode!(Coordinates, "X", "Y");

impl_xy_partial_gerbercode!(CoordinateOffset, "I", "J");

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

impl<W: Write> GerberCode<W> for DCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            DCode::Operation(ref operation) => operation.serialize(writer)?,
            DCode::SelectAperture(code) => write!(writer, "D{}*\n", code)?,
        };
        Ok(())
    }
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

impl<W: Write> GerberCode<W> for QuadrantMode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            QuadrantMode::Single => write!(writer, "G74*\n")?,
            QuadrantMode::Multi => write!(writer, "G75*\n")?,
        };
        Ok(())
    }
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

impl<W: Write> GerberCode<W> for MCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            MCode::EndOfFile => write!(writer, "M02*\n")?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Unit {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Unit::Millimeters => write!(writer, "MM")?,
            Unit::Inches => write!(writer, "IN")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Command {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Command::FunctionCode(ref code) => code.serialize(writer)?,
            Command::ExtendedCode(ref code) => code.serialize(writer)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for FunctionCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FunctionCode::DCode(ref code) => code.serialize(writer)?,
            FunctionCode::GCode(ref code) => code.serialize(writer)?,
            FunctionCode::MCode(ref code) => code.serialize(writer)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for ExtendedCode {
    fn serialize(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            ExtendedCode::CoordinateFormat(ref cf) => {
                write!(writer, "%FSLAX{0}{1}Y{0}{1}*%\n", cf.integer, cf.decimal)?;
            }
            ExtendedCode::Unit(ref unit) => {
                write!(writer, "%MO")?;
                unit.serialize_partial(writer)?;
                write!(writer, "*%\n")?;
            },
            ExtendedCode::ApertureDefinition(ref def) => {
                write!(writer, "%ADD")?;
                def.serialize_partial(writer)?;
                write!(writer, "*%\n")?;
            },
            ExtendedCode::ApertureMacro(ref am) => {
                write!(writer, "%")?;
                am.serialize_partial(writer)?;
                write!(writer, "%\n")?;
            },
            ExtendedCode::LoadPolarity(ref polarity) => {
                write!(writer, "%LP")?;
                polarity.serialize_partial(writer)?;
                write!(writer, "*%\n")?;
            },
            ExtendedCode::StepAndRepeat(ref sar) => {
                write!(writer, "%SR")?;
                sar.serialize_partial(writer)?;
                write!(writer, "*%\n")?;
            },
            ExtendedCode::FileAttribute(ref attr) => {
                write!(writer, "%TF.")?;
                attr.serialize_partial(writer)?;
                write!(writer, "*%\n")?;
            },
            ExtendedCode::DeleteAttribute(ref attr) => {
                write!(writer, "%TD{}*%\n", attr)?;
            },
            _ => unimplemented!(),
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for ApertureDefinition {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        write!(writer, "{}", self.code)?;
        self.aperture.serialize_partial(writer)?;
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Circle {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match self.hole_diameter {
            Some(hole_diameter) => {
                write!(writer, "{}X{}", self.diameter, hole_diameter)?;
            },
            None => write!(writer, "{}", self.diameter)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Rectangular {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match self.hole_diameter {
            Some(hole_diameter) => write!(writer, "{}X{}X{}", self.x, self.y, hole_diameter)?,
            None => write!(writer, "{}X{}", self.x, self.y)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Polygon {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match (self.rotation, self.hole_diameter) {
            (Some(rot), Some(hd)) => write!(writer, "{}X{}X{}X{}", self.diameter, self.vertices, rot, hd)?,
            (Some(rot), None) => write!(writer, "{}X{}X{}", self.diameter, self.vertices, rot)?,
            (None, Some(hd)) => write!(writer, "{}X{}X0X{}", self.diameter, self.vertices, hd)?,
            (None, None) => write!(writer, "{}X{}", self.diameter, self.vertices)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Aperture {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Aperture::Circle(ref circle) => {
                write!(writer, "C,")?;
                circle.serialize_partial(writer)?;
            },
            Aperture::Rectangle(ref rectangular) => {
                write!(writer, "R,")?;
                rectangular.serialize_partial(writer)?;
            },
            Aperture::Obround(ref rectangular) => {
                write!(writer, "O,")?;
                rectangular.serialize_partial(writer)?;
            },
            Aperture::Polygon(ref polygon) => {
                write!(writer, "P,")?;
                polygon.serialize_partial(writer)?;
            },
            Aperture::Other(ref string) => write!(writer, "{}", string)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Polarity {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Polarity::Clear => write!(writer, "C")?,
            Polarity::Dark => write!(writer, "D")?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for StepAndRepeat {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            StepAndRepeat::Open { repeat_x: rx, repeat_y: ry, distance_x: dx, distance_y: dy } =>
                write!(writer, "X{}Y{}I{}J{}", rx, ry, dx, dy)?,
            StepAndRepeat::Close => {},
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for Part {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Part::Single => write!(writer, "Single")?,
            Part::Array => write!(writer, "Array")?,
            Part::FabricationPanel => write!(writer, "FabricationPanel")?,
            Part::Coupon => write!(writer, "Coupon")?,
            Part::Other(ref description) => write!(writer, "Other,{}", description)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for GenerationSoftware {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match self.version {
            Some(ref v) => write!(writer, "{},{},{}", self.vendor, self.application, v)?,
            None => write!(writer, "{},{}", self.vendor, self.application)?,
        };
        Ok(())
    }
}

impl<W: Write> PartialGerberCode<W> for FileAttribute {
    fn serialize_partial(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FileAttribute::Part(ref part) => {
                write!(writer, "Part,")?;
                part.serialize_partial(writer)?;
            },
            FileAttribute::FileFunction(ref function) => {
                write!(writer, "FileFunction,")?;
                match function {
                    &FileFunction::Copper { ref layer, ref pos, ref copper_type } => {
                        write!(writer, "Copper,L{},", layer)?;
                        pos.serialize_partial(writer)?;
                        if let Some(ref t) = *copper_type {
                            write!(writer, ",")?;
                            t.serialize_partial(writer)?;
                        }
                    },
                    _ => unimplemented!(),
                }
            },
            FileAttribute::GenerationSoftware(ref gs) => {
                write!(writer, "GenerationSoftware,")?;
                gs.serialize_partial(writer)?;
            },
            FileAttribute::Md5(ref hash) => write!(writer, "MD5,{}", hash)?,
            _ => unimplemented!(),
        };
        Ok(())
    }
}
