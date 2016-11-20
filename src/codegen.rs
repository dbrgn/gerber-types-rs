use types::*;
use attributes::*;
use ::GerberResult;

/// All types that implement this trait can be converted to Gerber Code.
pub trait GerberCode {
    fn to_code(&self) -> GerberResult<String>;
}

/// Implement GerberCode for Vectors of types that implement GerberCode.
impl<T: GerberCode> GerberCode for Vec<T> {
    fn to_code(&self) -> GerberResult<String> {
        // Note: This can probably be implemented more efficently,
        // but we'll replace the `String` return type anways.
        let items = self.iter()
              .map(|g| g.to_code())
              .collect::<Vec<GerberResult<String>>>();
        let mut code = String::new();
        let mut first = true;
        for item in items {
            if first {
                first = false;
            } else {
                code.push_str("\n");
            }
            code.push_str(&try!(item));
        }
        Ok(code)
    }
}

/// Implement GerberCode for Option<T: GerberCode>
impl<T: GerberCode> GerberCode for Option<T> {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Some(ref v) => try!(v.to_code()),
            None => "".to_string(),
        };
        Ok(code)
    }
}

/// Automatically implement GerberCode trait for struct types
/// that are based on x and y attributes.
macro_rules! impl_xy_gerbercode {
    ($class:ty, $x:expr, $y: expr) => {
        impl GerberCode for $class {
            fn to_code(&self) -> GerberResult<String> {
                let mut code = String::new();
                if let Some(x) = self.x {
                    code = format!("{}{}", $x, try!(x.gerber(&self.format)));
                }
                if let Some(y) = self.y {
                    code.push_str(&format!("{}{}", $y, try!(y.gerber(&self.format))));
                }
                Ok(code)
            }
        }
    }
}

impl_xy_gerbercode!(Coordinates, "X", "Y");

impl_xy_gerbercode!(CoordinateOffset, "I", "J");

impl GerberCode for Operation {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Operation::Interpolate(ref coords, ref offset) => {
                format!("{}{}D01*", try!(coords.to_code()), try!(offset.to_code()))
            },
            Operation::Move(ref coords) => format!("{}D02*", try!(coords.to_code())),
            Operation::Flash(ref coords) => format!("{}D03*", try!(coords.to_code())),
        };
        Ok(code)
    }
}

impl GerberCode for DCode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            DCode::Operation(ref operation) => try!(operation.to_code()),
            DCode::SelectAperture(code) => format!("D{}*", code),
        };
        Ok(code)
    }
}

impl GerberCode for InterpolationMode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            InterpolationMode::Linear => "G01*",
            InterpolationMode::ClockwiseCircular => "G02*",
            InterpolationMode::CounterclockwiseCircular => "G03*",
        }.to_string();
        Ok(code)
    }
}

impl GerberCode for QuadrantMode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            QuadrantMode::Single => "G74*",
            QuadrantMode::Multi => "G75*",
        }.to_string();
        Ok(code)
    }
}

impl GerberCode for GCode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            GCode::InterpolationMode(ref mode) => try!(mode.to_code()),
            GCode::RegionMode(enabled) => if enabled { "G36*".to_string() } else { "G37*".to_string() },
            GCode::QuadrantMode(ref mode) => try!(mode.to_code()),
            GCode::Comment(ref comment) => format!("G04 {} *", comment),
        };
        Ok(code)
    }
}

impl GerberCode for MCode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            MCode::EndOfFile => "M02*",
        }.to_string();
        Ok(code)
    }
}

impl GerberCode for Unit {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Unit::Millimeters => "MM",
            Unit::Inches => "IN",
        }.to_string();
        Ok(code)
    }
}

impl GerberCode for FunctionCode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            FunctionCode::DCode(ref code) => try!(code.to_code()),
            FunctionCode::GCode(ref code) => try!(code.to_code()),
            FunctionCode::MCode(ref code) => try!(code.to_code()),
        };
        Ok(code)
    }
}

impl GerberCode for Command {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Command::FunctionCode(ref code) => try!(code.to_code()),
            Command::ExtendedCode(ref code) => try!(code.to_code()),
        };
        Ok(code)
    }
}

impl GerberCode for ExtendedCode {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            ExtendedCode::CoordinateFormat(ref cf) => format!("%FSLAX{0}{1}Y{0}{1}*%", cf.integer, cf.decimal),
            ExtendedCode::Unit(ref unit) => format!("%MO{}*%", try!(unit.to_code())),
            ExtendedCode::ApertureDefinition(ref def) => format!("%ADD{}*%", try!(def.to_code())),
            ExtendedCode::ApertureMacro => panic!("not yet implemented"),
            ExtendedCode::LoadPolarity(ref polarity) => format!("%LP{}*%", try!(polarity.to_code())),
            ExtendedCode::StepAndRepeat(ref sar) => format!("%SR{}*%", try!(sar.to_code())),
            ExtendedCode::FileAttribute(ref attr) => format!("%TF.{}*%", try!(attr.to_code())),
            //ExtendedCode::ApertureAttribute(ref attr) => ,
            ExtendedCode::DeleteAttribute(ref attr) => format!("%TD{}*%", attr),
            _ => panic!("not yet implemented"),
        };
        Ok(code)
    }
}

impl GerberCode for ApertureDefinition {
    fn to_code(&self) -> GerberResult<String> {
        Ok(format!("{}{}", self.code, try!(self.aperture.to_code())))
    }
}

impl GerberCode for Circle {
    fn to_code(&self) -> GerberResult<String> {
        let code = match self.hole_diameter {
            Some(hole_diameter) => format!("{}X{}", self.diameter, hole_diameter),
            None => format!("{}", self.diameter),
        };
        Ok(code)
    }
}

impl GerberCode for Rectangular {
    fn to_code(&self) -> GerberResult<String> {
        let code = match self.hole_diameter {
            Some(hole_diameter) => format!("{}X{}X{}", self.x, self.y, hole_diameter),
            None => format!("{}X{}", self.x, self.y),
        };
        Ok(code)
    }
}

impl GerberCode for Polygon {
    fn to_code(&self) -> GerberResult<String> {
        let code = match (self.rotation, self.hole_diameter) {
            (Some(rot), Some(hd)) => format!("{}X{}X{}X{}", self.diameter, self.vertices, rot, hd),
            (Some(rot), None) => format!("{}X{}X{}", self.diameter, self.vertices, rot),
            (None, Some(hd)) => format!("{}X{}X0X{}", self.diameter, self.vertices, hd),
            (None, None) => format!("{}X{}", self.diameter, self.vertices),
        };
        Ok(code)
    }
}

impl GerberCode for Aperture {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Aperture::Circle(ref circle) => format!("C,{}", try!(circle.to_code())),
            Aperture::Rectangle(ref rectangular) => format!("R,{}", try!(rectangular.to_code())),
            Aperture::Obround(ref rectangular) => format!("O,{}", try!(rectangular.to_code())),
            Aperture::Polygon(ref polygon) => format!("P,{}", try!(polygon.to_code())),
            Aperture::Other(ref string) => panic!("not yet implemented"),
        };
        Ok(code)
    }
}

impl GerberCode for Polarity {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Polarity::Clear => "C".to_string(),
            Polarity::Dark => "D".to_string(),
        };
        Ok(code)
    }
}

impl GerberCode for StepAndRepeat {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            StepAndRepeat::Open { repeat_x: rx, repeat_y: ry, distance_x: dx, distance_y: dy } =>
                format!("X{}Y{}I{}J{}", rx, ry, dx, dy),
            StepAndRepeat::Close => String::new(),
        };
        Ok(code)
    }
}

impl GerberCode for Part {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Part::Single => "Single".into(),
            Part::Array => "Array".into(),
            Part::FabricationPanel => "FabricationPanel".into(),
            Part::Coupon => "Coupon".into(),
            Part::Other(ref description) => format!("Other,{}", description),
        };
        Ok(code)
    }
}

impl GerberCode for FileAttribute {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            FileAttribute::Part(ref part) => format!("Part,{}", try!(part.to_code())),
            _ => panic!("Not yet implemented"),
        };
        Ok(code)
    }
}
