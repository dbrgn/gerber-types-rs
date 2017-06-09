use std::io::Write;

use types::*;
use attributes::*;
use ::GerberResult;

/// All types that implement this trait can be converted to Gerber Code.
pub trait GerberCode<W: Write> {
    fn to_code(&self, writer: &mut W) -> GerberResult<()>;
}

/// Implement `GerberCode` for booleans
impl<W: Write> GerberCode<W> for bool {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        if *self {
            write!(writer, "1")?;
        } else {
            write!(writer, "0")?;
        };
        Ok(())
    }
}

/// Implement `GerberCode` for Vectors of commands.
impl<W: Write> GerberCode<W> for Vec<Command> {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        let mut first = true;
        for item in self.iter() {
            if first {
                first = false;
            } else {
                write!(writer, "\n")?;
            }
            item.to_code(writer)?;
        }
        Ok(())
    }
}

/// Implement `GerberCode` for `Option<T: GerberCode>`
impl<T: GerberCode<W>, W: Write> GerberCode<W> for Option<T> {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        if let Some(ref val) = *self {
            val.to_code(writer)?;
        }
        Ok(())
    }
}

/// Automatically implement `GerberCode` trait for struct types
/// that are based on `x` and `y` attributes.
macro_rules! impl_xy_gerbercode {
    ($class:ty, $x:expr, $y: expr) => {
        impl<W: Write> GerberCode<W> for $class {
            fn to_code(&self, writer: &mut W) -> GerberResult<()> {
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

impl_xy_gerbercode!(Coordinates, "X", "Y");

impl_xy_gerbercode!(CoordinateOffset, "I", "J");

impl<W: Write> GerberCode<W> for Operation {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Operation::Interpolate(ref coords, ref offset) => {
                coords.to_code(writer)?;
                offset.to_code(writer)?;
                write!(writer, "D01*")?;
            },
            Operation::Move(ref coords) => {
                coords.to_code(writer)?;
                write!(writer, "D02*")?;
            },
            Operation::Flash(ref coords) => {
                coords.to_code(writer)?;
                write!(writer, "D03*")?;
            }
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for DCode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            DCode::Operation(ref operation) => operation.to_code(writer)?,
            DCode::SelectAperture(code) => write!(writer, "D{}*", code)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for InterpolationMode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            InterpolationMode::Linear => write!(writer, "G01*")?,
            InterpolationMode::ClockwiseCircular => write!(writer, "G02*")?,
            InterpolationMode::CounterclockwiseCircular => write!(writer, "G03*")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for QuadrantMode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            QuadrantMode::Single => write!(writer, "G74*")?,
            QuadrantMode::Multi => write!(writer, "G75*")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for GCode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            GCode::InterpolationMode(ref mode) => mode.to_code(writer)?,
            GCode::RegionMode(enabled) => if enabled {
                write!(writer, "G36*")?;
            } else {
                write!(writer, "G37*")?;
            },
            GCode::QuadrantMode(ref mode) => mode.to_code(writer)?,
            GCode::Comment(ref comment) => write!(writer, "G04 {} *", comment)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for MCode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            MCode::EndOfFile => write!(writer, "M02*")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Unit {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Unit::Millimeters => write!(writer, "MM")?,
            Unit::Inches => write!(writer, "IN")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Command {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Command::FunctionCode(ref code) => code.to_code(writer)?,
            Command::ExtendedCode(ref code) => code.to_code(writer)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for FunctionCode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FunctionCode::DCode(ref code) => code.to_code(writer)?,
            FunctionCode::GCode(ref code) => code.to_code(writer)?,
            FunctionCode::MCode(ref code) => code.to_code(writer)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for ExtendedCode {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            ExtendedCode::CoordinateFormat(ref cf) => {
                write!(writer, "%FSLAX{0}{1}Y{0}{1}*%", cf.integer, cf.decimal)?;
            }
            ExtendedCode::Unit(ref unit) => {
                write!(writer, "%MO")?;
                unit.to_code(writer)?;
                write!(writer, "*%")?;
            },
            ExtendedCode::ApertureDefinition(ref def) => {
                write!(writer, "%ADD")?;
                def.to_code(writer)?;
                write!(writer, "*%")?;
            },
            ExtendedCode::ApertureMacro(ref am) => {
                write!(writer, "%")?;
                am.to_code(writer)?;
                write!(writer, "%")?;
            },
            ExtendedCode::LoadPolarity(ref polarity) => {
                write!(writer, "%LP")?;
                polarity.to_code(writer)?;
                write!(writer, "*%")?;
            },
            ExtendedCode::StepAndRepeat(ref sar) => {
                write!(writer, "%SR")?;
                sar.to_code(writer)?;
                write!(writer, "*%")?;
            },
            ExtendedCode::FileAttribute(ref attr) => {
                write!(writer, "%TF.")?;
                attr.to_code(writer)?;
                write!(writer, "*%")?;
            },
            ExtendedCode::DeleteAttribute(ref attr) => {
                write!(writer, "%TD{}*%", attr)?;
            },
            _ => unimplemented!(),
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for ApertureDefinition {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        write!(writer, "{}", self.code)?;
        self.aperture.to_code(writer)?;
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Circle {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match self.hole_diameter {
            Some(hole_diameter) => {
                write!(writer, "{}X{}", self.diameter, hole_diameter)?;
            },
            None => write!(writer, "{}", self.diameter)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Rectangular {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match self.hole_diameter {
            Some(hole_diameter) => write!(writer, "{}X{}X{}", self.x, self.y, hole_diameter)?,
            None => write!(writer, "{}X{}", self.x, self.y)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Polygon {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match (self.rotation, self.hole_diameter) {
            (Some(rot), Some(hd)) => write!(writer, "{}X{}X{}X{}", self.diameter, self.vertices, rot, hd)?,
            (Some(rot), None) => write!(writer, "{}X{}X{}", self.diameter, self.vertices, rot)?,
            (None, Some(hd)) => write!(writer, "{}X{}X0X{}", self.diameter, self.vertices, hd)?,
            (None, None) => write!(writer, "{}X{}", self.diameter, self.vertices)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Aperture {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Aperture::Circle(ref circle) => {
                write!(writer, "C,")?;
                circle.to_code(writer)?;
            },
            Aperture::Rectangle(ref rectangular) => {
                write!(writer, "R,")?;
                rectangular.to_code(writer)?;
            },
            Aperture::Obround(ref rectangular) => {
                write!(writer, "O,")?;
                rectangular.to_code(writer)?;
            },
            Aperture::Polygon(ref polygon) => {
                write!(writer, "P,")?;
                polygon.to_code(writer)?;
            },
            Aperture::Other(ref string) => write!(writer, "{}", string)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Polarity {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            Polarity::Clear => write!(writer, "C")?,
            Polarity::Dark => write!(writer, "D")?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for StepAndRepeat {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            StepAndRepeat::Open { repeat_x: rx, repeat_y: ry, distance_x: dx, distance_y: dy } =>
                write!(writer, "X{}Y{}I{}J{}", rx, ry, dx, dy)?,
            StepAndRepeat::Close => {},
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for Part {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
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

impl<W: Write> GerberCode<W> for GenerationSoftware {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match self.version {
            Some(ref v) => write!(writer, "{},{},{}", self.vendor, self.application, v)?,
            None => write!(writer, "{},{}", self.vendor, self.application)?,
        };
        Ok(())
    }
}

impl<W: Write> GerberCode<W> for FileAttribute {
    fn to_code(&self, writer: &mut W) -> GerberResult<()> {
        match *self {
            FileAttribute::Part(ref part) => {
                write!(writer, "Part,")?;
                part.to_code(writer)?;
            },
            FileAttribute::GenerationSoftware(ref gs) => {
                write!(writer, "GenerationSoftware,")?;
                gs.to_code(writer)?;
            },
            FileAttribute::Md5(ref hash) => write!(writer, "MD5,{}", hash)?,
            _ => unimplemented!(),
        };
        Ok(())
    }
}
