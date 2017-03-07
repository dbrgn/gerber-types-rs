//! Aperture Macros.

use std::convert::From;

use ::{GerberCode, GerberError, GerberResult};

#[derive(Debug)]
pub struct ApertureMacro {
    pub name: String,
    pub primitives: Vec<Primitive>,
}

impl ApertureMacro {
    pub fn new<S: Into<String>>(name: S) -> Self {
        ApertureMacro {
            name: name.into(),
            primitives: Vec::new(),
        }
    }

    pub fn add_primitive(mut self, p: Primitive) -> Self {
        self.primitives.push(p);
        self
    }

    pub fn add_primitive_mut(&mut self, p: Primitive) {
        self.primitives.push(p);
    }
}

impl GerberCode for ApertureMacro {
    fn to_code(&self) -> GerberResult<String> {
        if self.primitives.len() == 0 {
            return Err(GerberError::MissingDataError("There must be at least 1 primitive in an aperture macro".into()));
        }
        let primitives = self.primitives.iter()
                                        .map(|p| p.to_code())
                                        .collect::<GerberResult<Vec<String>>>()?;
        Ok(format!("AM{}*\n{}", self.name, primitives.join("\n")))
    }
}

#[derive(Debug, PartialEq)]
/// A macro decimal can either be an f64 or a variable placeholder.
pub enum MacroDecimal {
    /// A decimal value.
    Value(f64),
    /// A variable placeholder.
    Variable(u8),
}

impl MacroDecimal {
    fn is_negative(&self) -> bool {
        match *self {
            MacroDecimal::Value(v) => v < 0.0,
            MacroDecimal::Variable(_) => false,
        }
    }
}

impl From<f32> for MacroDecimal {
    fn from(val: f32) -> Self {
        MacroDecimal::Value(val as f64)
    }
}

impl From<f64> for MacroDecimal {
    fn from(val: f64) -> Self {
        MacroDecimal::Value(val)
    }
}

impl GerberCode for MacroDecimal {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            MacroDecimal::Value(ref v) => format!("{}", v),
            MacroDecimal::Variable(ref v) => format!("${}", v),
        };
        Ok(code)
    }
}

#[derive(Debug)]
pub enum Primitive {
    Comment(String),
    Circle(CirclePrimitive),
    VectorLine(VectorLinePrimitive),
    CenterLine(CenterLinePrimitive),
    Outline(OutlinePrimitive),
    Polygon(PolygonPrimitive),
    Moire(MoirePrimitive),
    Thermal(ThermalPrimitive),
}

impl GerberCode for Primitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Primitive::Comment(ref s) => format!("0 {}*", &s),
            Primitive::Circle(ref c) => try!(c.to_code()),
            Primitive::VectorLine(ref vl) => try!(vl.to_code()),
            Primitive::CenterLine(ref cl) => try!(cl.to_code()),
            Primitive::Outline(ref o) => try!(o.to_code()),
            Primitive::Polygon(ref p) => try!(p.to_code()),
            Primitive::Moire(ref m) => try!(m.to_code()),
            Primitive::Thermal(ref t) => try!(t.to_code()),
        };
        Ok(code)
    }
}

#[derive(Debug)]
pub struct CirclePrimitive {
    /// Exposure off/on
    pub exposure: bool,

    /// Diameter, a decimal >= 0
    pub diameter: MacroDecimal,

    /// X and Y coordinates of center position, decimals
    pub center: (MacroDecimal, MacroDecimal),

    /// Rotation angle.
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    ///
    /// The rotation modifier is optional. The default is no rotation. (We
    /// recommend always to set the angle explicitly.
    pub angle: Option<MacroDecimal>,
}

impl GerberCode for CirclePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let mut code = "1,".to_string();
        code.push_str(&try!(self.exposure.to_code()));
        code.push_str(&format!(",{},{},{}", self.diameter.to_code()?, self.center.0.to_code()?, self.center.1.to_code()?));
        if let Some(ref a) = self.angle {
            code.push_str(&format!(",{}", a.to_code()?));
        }
        code.push_str("*");
        Ok(code)
    }
}

#[derive(Debug)]
pub struct VectorLinePrimitive {
    /// Exposure off/on
    pub exposure: bool,

    /// Line width, a decimal >= 0
    pub width: MacroDecimal,

    /// X and Y coordinates of start point, decimals
    pub start: (MacroDecimal, MacroDecimal),

    /// X and Y coordinates of end point, decimals
    pub end: (MacroDecimal, MacroDecimal),

    /// Rotation angle of the vector line primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    pub angle: MacroDecimal,
}

impl GerberCode for VectorLinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = format!(
            "20,{},{},{},{},{},{},{}*",
            self.exposure.to_code()?,
            self.width.to_code()?,
            self.start.0.to_code()?, self.start.1.to_code()?,
            self.end.0.to_code()?, self.end.1.to_code()?,
            self.angle.to_code()?
        );
        Ok(code)
    }
}

#[derive(Debug)]
pub struct CenterLinePrimitive {
    /// Exposure off/on (0/1)
    pub exposure: bool,

    /// Rectangle dimensions (width/height)
    pub dimensions: (MacroDecimal, MacroDecimal),

    /// X and Y coordinates of center point, decimals
    pub center: (MacroDecimal, MacroDecimal),

    /// Rotation angle
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. (0, 0) point
    /// of macro coordinates.
    pub angle: MacroDecimal,
}

impl GerberCode for CenterLinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = format!(
            "21,{},{},{},{},{},{}*",
            try!(self.exposure.to_code()),
            self.dimensions.0.to_code()?, self.dimensions.1.to_code()?,
            self.center.0.to_code()?, self.center.1.to_code()?,
            self.angle.to_code()?
        );
        Ok(code)
    }
}

#[derive(Debug)]
pub struct OutlinePrimitive {
    /// Exposure off/on (0/1)
    pub exposure: bool,

    /// Vector of coordinate pairs.
    ///
    /// The last coordinate pair must be equal to the first coordinate pair!
    pub points: Vec<(MacroDecimal, MacroDecimal)>,

    /// Rotation angle of the outline primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    pub angle: MacroDecimal,
}

impl GerberCode for OutlinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Points invariants
        if self.points.len() < 2 {
            return Err(GerberError::MissingDataError("There must be at least 1 subsequent point in an outline".into()));
        }
        if self.points.len() > 5001 {
            return Err(GerberError::RangeError("The maximum number of subsequent points in an outline is 5000".into()));
        }
        if self.points[0] != self.points[self.points.len() - 1] {
            return Err(GerberError::RangeError("The last point must be equal to the first point".into()));
        }

        let mut code = format!("4,{},{},\n", try!(self.exposure.to_code()), self.points.len() - 1);
        let points = self.points.iter()
                         .map(|&(ref x, ref y)| Ok(format!("{},{},", x.to_code()?, y.to_code()?)))
                         .collect::<GerberResult<Vec<String>>>()?;
        code.push_str(&points.join("\n"));
        code.push_str(&format!("\n{}*", self.angle.to_code()?));
        Ok(code)
    }
}

#[derive(Debug)]
/// A polygon primitive is a regular polygon defined by the number of vertices,
/// the center point and the diameter of the circumscribed circle.
pub struct PolygonPrimitive {
    /// Exposure off/on (0/1)
    pub exposure: bool,

    /// Number of vertices n, 3 <= n <= 12
    pub vertices: u8,

    /// X and Y coordinates of center point, decimals
    pub center: (MacroDecimal, MacroDecimal),

    /// Diameter of the circumscribed circle, a decimal >= 0
    pub diameter: MacroDecimal,

    /// Rotation angle of the polygon primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates. The first vertex is on the positive X-axis
    /// through the center point when the rotation angle is zero.
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: MacroDecimal,
}

impl GerberCode for PolygonPrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Vertice count invariants
        if self.vertices < 3 {
            return Err(GerberError::MissingDataError("There must be at least 3 vertices in a polygon".into()));
        }
        if self.vertices > 12 {
            return Err(GerberError::RangeError("The maximum number of vertices in a polygon is 12".into()));
        }
        if self.diameter.is_negative() {
            return Err(GerberError::RangeError("The diameter must not be negative".into()));
        }
        let code = format!(
            "5,{},{},{},{},{},{}*",
            self.exposure.to_code()?,
            self.vertices,
            self.center.0.to_code()?, self.center.1.to_code()?,
            self.diameter.to_code()?,
            self.angle.to_code()?
        );
        Ok(code)
    }
}

/// The moiré primitive is a cross hair centered on concentric rings (annuli).
/// Exposure is always on.
#[derive(Debug)]
pub struct MoirePrimitive {
    /// X and Y coordinates of center point, decimals
    pub center: (MacroDecimal, MacroDecimal),

    /// Outer diameter of outer concentric ring, a decimal >= 0
    pub diameter: MacroDecimal,

    /// Ring thickness, a decimal >= 0
    pub ring_thickness: MacroDecimal,

    /// Gap between rings, a decimal >= 0
    pub gap: MacroDecimal,

    /// Maximum number of rings
    pub max_rings: u32,

    /// Cross hair thickness, a decimal >= 0
    pub cross_hair_thickness: MacroDecimal,

    /// Cross hair length, a decimal >= 0
    pub cross_hair_length: MacroDecimal,

    /// Rotation angle of the moiré primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: MacroDecimal,
}

impl GerberCode for MoirePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Decimal invariants
        if self.diameter.is_negative() {
            return Err(GerberError::RangeError("Outer diameter of a moiré may not be negative".into()));
        }
        if self.ring_thickness.is_negative() {
            return Err(GerberError::RangeError("Ring thickness of a moiré may not be negative".into()));
        }
        if self.gap.is_negative() {
            return Err(GerberError::RangeError("Gap of a moiré may not be negative".into()));
        }
        if self.cross_hair_thickness.is_negative() {
            return Err(GerberError::RangeError("Cross hair thickness of a moiré may not be negative".into()));
        }
        if self.cross_hair_length.is_negative() {
            return Err(GerberError::RangeError("Cross hair length of a moiré may not be negative".into()));
        }
        let code = format!(
            "6,{},{},{},{},{},{},{},{},{}*",
            self.center.0.to_code()?, self.center.1.to_code()?,
            self.diameter.to_code()?,
            self.ring_thickness.to_code()?,
            self.gap.to_code()?,
            self.max_rings,
            self.cross_hair_thickness.to_code()?, self.cross_hair_length.to_code()?,
            self.angle.to_code()?
        );
        Ok(code)
    }
}

/// The thermal primitive is a ring (annulus) interrupted by four gaps.
/// Exposure is always on.
#[derive(Debug)]
pub struct ThermalPrimitive {
    /// X and Y coordinates of center point, decimals
    pub center: (MacroDecimal, MacroDecimal),

    /// Outer diameter, a decimal > inner diameter
    pub outer_diameter: MacroDecimal,

    /// Inner diameter, a decimal >= 0
    pub inner_diameter: MacroDecimal,

    /// Gap thickness, a decimal < (outer diameter) / sqrt(2)
    pub gap: MacroDecimal,

    /// Rotation angle of the thermal primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates. The gaps are on the X and Y axes through
    /// the center when the rotation angle is zero
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: MacroDecimal,
}

impl GerberCode for ThermalPrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Decimal invariants
        if self.inner_diameter.is_negative() {
            return Err(GerberError::RangeError("Inner diameter of a thermal may not be negative".into()));
        }
        let code = format!(
            "7,{},{},{},{},{},{}*",
            self.center.0.to_code()?, self.center.1.to_code()?,
            self.outer_diameter.to_code()?,
            self.inner_diameter.to_code()?,
            self.gap.to_code()?,
            self.angle.to_code()?
        );
        Ok(code)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::MacroDecimal::{Value, Variable};
    use ::GerberCode;

    #[test]
    fn test_circle_primitive_codegen() {
        let with_angle = CirclePrimitive {
            exposure: true,
            diameter: Value(1.5),
            center: (Value(0.), Value(0.)),
            angle: Some(Value(0.)),
        };
        assert_eq!(with_angle.to_code().unwrap(), "1,1,1.5,0,0,0*".to_string());
        let no_angle = CirclePrimitive {
            exposure: false,
            diameter: Value(99.9),
            center: (Value(1.1), Value(2.2)),
            angle: None,
        };
        assert_eq!(no_angle.to_code().unwrap(), "1,0,99.9,1.1,2.2*".to_string());
    }

    #[test]
    fn test_vector_line_primitive_codegen() {
        let line = VectorLinePrimitive {
            exposure: true,
            width: Value(0.9),
            start: (Value(0.), Value(0.45)),
            end: (Value(12.), Value(0.45)),
            angle: Value(0.),
        };
        assert_eq!(line.to_code().unwrap(), "20,1,0.9,0,0.45,12,0.45,0*".to_string());
    }

    #[test]
    fn test_center_line_primitive_codegen() {
        let line = CenterLinePrimitive {
            exposure: true,
            dimensions: (Value(6.8), Value(1.2)),
            center: (Value(3.4), Value(0.6)),
            angle: Value(30.0),
        };
        assert_eq!(line.to_code().unwrap(), "21,1,6.8,1.2,3.4,0.6,30*".to_string());
    }

    #[test]
    fn test_outline_primitive_codegen() {
        let line = OutlinePrimitive {
            exposure: true,
            points: vec![
                (Value(0.1), Value(0.1)),
                (Value(0.5), Value(0.1)),
                (Value(0.5), Value(0.5)),
                (Value(0.1), Value(0.5)),
                (Value(0.1), Value(0.1)),
            ],
            angle: Value(0.0),
        };
        assert_eq!(line.to_code().unwrap(), "4,1,4,\n0.1,0.1,\n0.5,0.1,\n0.5,0.5,\n0.1,0.5,\n0.1,0.1,\n0*".to_string());
    }

    #[test]
    fn test_polygon_primitive_codegen() {
        let line = PolygonPrimitive {
            exposure: true,
            vertices: 8,
            center: (Value(1.5), Value(2.0)),
            diameter: Value(8.0),
            angle: Value(0.0),
        };
        assert_eq!(line.to_code().unwrap(), "5,1,8,1.5,2,8,0*".to_string());
    }

    #[test]
    fn test_moire_primitive_codegen() {
        let line = MoirePrimitive {
            center: (Value(0.0), Value(0.0)),
            diameter: Value(5.0),
            ring_thickness: Value(0.5),
            gap: Value(0.5),
            max_rings: 2,
            cross_hair_thickness: Value(0.1),
            cross_hair_length: Value(6.0),
            angle: Value(0.0),
        };
        assert_eq!(line.to_code().unwrap(), "6,0,0,5,0.5,0.5,2,0.1,6,0*".to_string());
    }

    #[test]
    fn test_thermal_primitive_codegen() {
        let line = ThermalPrimitive {
            center: (Value(0.0), Value(0.0)),
            outer_diameter: Value(8.0),
            inner_diameter: Value(6.5),
            gap: Value(1.0),
            angle: Value(45.0),
        };
        assert_eq!(line.to_code().unwrap(), "7,0,0,8,6.5,1,45*".to_string());
    }

    #[test]
    fn test_aperture_macro_codegen() {
        let am = ApertureMacro::new("CRAZY").add_primitive(
            Primitive::Thermal(
                ThermalPrimitive {
                    center: (Value(0.0), Value(0.0)),
                    outer_diameter: Value(0.08),
                    inner_diameter: Value(0.055),
                    gap: Value(0.0125),
                    angle: Value(45.0),
                }
            )
        ).add_primitive(
            Primitive::Moire(
                MoirePrimitive {
                    center: (Value(0.0), Value(0.0)),
                    diameter: Value(0.125),
                    ring_thickness: Value(0.01),
                    gap: Value(0.01),
                    max_rings: 3,
                    cross_hair_thickness: Value(0.003),
                    cross_hair_length: Value(0.150),
                    angle: Value(0.0),
                }
            )
        );
        assert_eq!(am.to_code().unwrap(), "AMCRAZY*\n7,0,0,0.08,0.055,0.0125,45*\n6,0,0,0.125,0.01,0.01,3,0.003,0.15,0*".to_string());
    }

    #[test]
    fn test_codegen_with_variable() {
        let line = VectorLinePrimitive {
            exposure: true,
            width: Variable(0),
            start: (Variable(1), 0.45.into()),
            end: (Value(12.), Variable(2)),
            angle: Variable(3),
        };
        assert_eq!(line.to_code().unwrap(), "20,1,$0,$1,0.45,12,$2,$3*".to_string());
    }

    #[test]
    fn test_macro_decimal_into() {
        let a = Value(1.0);
        let b: MacroDecimal = 1.0.into();
        assert_eq!(a, b);
        let c = Variable(1);
        let d = Variable(1);
        assert_eq!(c, d);
    }

    #[test]
    fn test_comment_codegen() {
        let comment = Primitive::Comment("hello world".to_string());
        assert_eq!(&comment.to_code().unwrap(), "0 hello world*");
    }
}
