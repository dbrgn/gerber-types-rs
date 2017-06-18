//! Generic code generation, e.g. implementations of `PartialGerberCode` for
//! bool or Vec<G: GerberCode>.

use std::io::Write;

use attributes::*;
use errors::GerberResult;
use traits::{GerberCode, PartialGerberCode};
use types::*;

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
