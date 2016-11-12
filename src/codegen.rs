use types::*;

/// All types that implement this trait can be converted to Gerber Code.
pub trait GerberCode {
    fn to_code(&self) -> String;
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

impl_xy_gerbercode!(Coordinates, "X", "Y");

impl_xy_gerbercode!(CoordinateOffset, "I", "J");

impl GerberCode for Operation {
    fn to_code(&self) -> String {
        match *self {
            Operation::Interpolate(ref coords, ref offset) => format!("{}{}D01*", coords.to_code(), offset.to_code()),
            Operation::Move(ref coords) => format!("{}D02*", coords.to_code()),
            Operation::Flash(ref coords) => format!("{}D03*", coords.to_code()),
        }
    }
}

impl GerberCode for DCode {
    fn to_code(&self) -> String {
        match *self {
            DCode::Operation(ref operation) => operation.to_code(),
            DCode::SelectAperture(code) => format!("D{}*", code),
        }
    }
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

impl GerberCode for QuadrantMode {
    fn to_code(&self) -> String {
        match *self {
            QuadrantMode::Single => "G74*",
            QuadrantMode::Multi => "G75*",
        }.to_string()
    }
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

impl GerberCode for MCode {
    fn to_code(&self) -> String {
        match *self {
            MCode::EndOfFile => "M02*",
        }.to_string()
    }
}

impl GerberCode for Unit {
    fn to_code(&self) -> String {
        match *self {
            Unit::Millimeters => "MM",
            Unit::Inches => "IN",
        }.to_string()
    }
}

impl GerberCode for FunctionCode {
    fn to_code(&self) -> String {
        match *self {
            FunctionCode::DCode(ref code) => code.to_code(),
            FunctionCode::GCode(ref code) => code.to_code(),
            FunctionCode::MCode(ref code) => code.to_code(),
        }
    }
}

impl GerberCode for ExtendedCode {
    fn to_code(&self) -> String {
        match *self {
            ExtendedCode::CoordinateFormat(ref x, ref y) => format!("%FSLAX{0}{1}Y{0}{1}*%", x, y),
            ExtendedCode::Unit(ref unit) => format!("%MO{}*%", unit.to_code()),
            ExtendedCode::ApertureDefinition(ref def) => format!("%ADD{}*%", def.to_code()),
            _ => panic!("not yet implemented"),
        }
    }
}

impl GerberCode for ApertureDefinition {
    fn to_code(&self) -> String {
        return format!("{}{}", self.code, self.aperture.to_code());
    }
}

impl GerberCode for Circle {
    fn to_code(&self) -> String {
        match self.hole_diameter {
            Some(hole_diameter) => format!("{}X{}", self.diameter, hole_diameter),
            None => format!("{}", self.diameter),
        }
    }
}

impl GerberCode for Rectangular {
    fn to_code(&self) -> String {
        match self.hole_diameter {
            Some(hole_diameter) => format!("{}X{}X{}", self.x, self.y, hole_diameter),
            None => format!("{}X{}", self.x, self.y),
        }
    }
}

impl GerberCode for Polygon {
    fn to_code(&self) -> String {
        match (self.rotation, self.hole_diameter) {
            (Some(rot), Some(hd)) => format!("{}X{}X{}X{}", self.diameter, self.vertices, rot, hd),
            (Some(rot), None) => format!("{}X{}X{}", self.diameter, self.vertices, rot),
            (None, Some(hd)) => format!("{}X{}X0X{}", self.diameter, self.vertices, hd),
            (None, None) => format!("{}X{}", self.diameter, self.vertices),
        }
    }
}

impl GerberCode for Aperture {
    fn to_code(&self) -> String {
        match *self {
            Aperture::Circle(ref circle) => format!("C,{}", circle.to_code()),
            Aperture::Rectangle(ref rectangular) => format!("R,{}", rectangular.to_code()),
            Aperture::Obround(ref rectangular) => format!("O,{}", rectangular.to_code()),
            Aperture::Polygon(ref polygon) => format!("P,{}", polygon.to_code()),
            Aperture::Other(ref string) => panic!("not yet implemented"),
        }
    }
}
