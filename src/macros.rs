//! Aperture Macros.

use std::convert::From;
use std::io::Write;

use ::{GerberCode, GerberError, GerberResult};

#[derive(Debug)]
pub struct ApertureMacro {
    pub name: String,
    pub content: Vec<MacroContent>,
}

impl ApertureMacro {
    pub fn new<S: Into<String>>(name: S) -> Self {
        ApertureMacro {
            name: name.into(),
            content: Vec::new(),
        }
    }

    pub fn add_content<C>(mut self, c: C) -> Self where C: Into<MacroContent>{
        self.content.push(c.into());
        self
    }

    pub fn add_content_mut<C>(&mut self, c: C) where C: Into<MacroContent> {
        self.content.push(c.into());
    }
}

impl<W: Write> GerberCode<W> for ApertureMacro {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        if self.content.len() == 0 {
            return Err(GerberError::MissingDataError("There must be at least 1 content element in an aperture macro".into()));
        }
        writeln!(writer, "AM{}*", self.name)?;
        let mut first = true;
        for content in self.content.iter() {
            if first {
                first = false;
            } else {
                writeln!(writer)?;
            }
            content.to_code(&mut writer)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
/// A macro decimal can either be an f64 or a variable placeholder.
pub enum MacroDecimal {
    /// A decimal value.
    Value(f64),
    /// A variable placeholder.
    Variable(u32),
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

impl<W: Write> GerberCode<W> for MacroDecimal {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        match *self {
            MacroDecimal::Value(ref v) => write!(writer, "{}", v)?,
            MacroDecimal::Variable(ref v) => write!(writer, "${}", v)?,
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum MacroContent {
    // Primitives
    Circle(CirclePrimitive),
    VectorLine(VectorLinePrimitive),
    CenterLine(CenterLinePrimitive),
    Outline(OutlinePrimitive),
    Polygon(PolygonPrimitive),
    Moire(MoirePrimitive),
    Thermal(ThermalPrimitive),

    // Variables
    VariableDefinition(VariableDefinition),

    // Comment
    Comment(String),
}

impl<W: Write> GerberCode<W> for MacroContent {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        match *self {
            MacroContent::Circle(ref c) => c.to_code(&mut writer)?,
            MacroContent::VectorLine(ref vl) => vl.to_code(&mut writer)?,
            MacroContent::CenterLine(ref cl) => cl.to_code(&mut writer)?,
            MacroContent::Outline(ref o) => o.to_code(&mut writer)?,
            MacroContent::Polygon(ref p) => p.to_code(&mut writer)?,
            MacroContent::Moire(ref m) => m.to_code(&mut writer)?,
            MacroContent::Thermal(ref t) => t.to_code(&mut writer)?,
            MacroContent::Comment(ref s) => write!(writer, "0 {}*", &s)?,
            MacroContent::VariableDefinition(ref v) => v.to_code(&mut writer)?,
        };
        Ok(())
    }
}

macro_rules! impl_into {
    ($target:ty, $from:ty, $choice:expr) => {
        impl From<$from> for $target {
            fn from(val: $from) -> $target {
                $choice(val)
            }
        }
    }
}

impl_into!(MacroContent, CirclePrimitive, MacroContent::Circle);
impl_into!(MacroContent, VectorLinePrimitive, MacroContent::VectorLine);
impl_into!(MacroContent, CenterLinePrimitive, MacroContent::CenterLine);
impl_into!(MacroContent, OutlinePrimitive, MacroContent::Outline);
impl_into!(MacroContent, PolygonPrimitive, MacroContent::Polygon);
impl_into!(MacroContent, MoirePrimitive, MacroContent::Moire);
impl_into!(MacroContent, ThermalPrimitive, MacroContent::Thermal);
impl_into!(MacroContent, VariableDefinition, MacroContent::VariableDefinition);

impl<T: Into<String>> From<T> for MacroContent {
    fn from(val: T) -> Self {
        MacroContent::Comment(val.into())
    }
}

#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for CirclePrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        write!(writer, "1,")?;
        self.exposure.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.diameter.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.1.to_code(&mut writer)?;
        if let Some(ref a) = self.angle {
            write!(writer, ",")?;
            a.to_code(&mut writer)?;
        }
        write!(writer, "*")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for VectorLinePrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        write!(writer, "20,")?;
        self.exposure.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.width.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.start.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.start.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.end.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.end.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for CenterLinePrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        write!(writer, "21,")?;
        self.exposure.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.dimensions.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.dimensions.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for OutlinePrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
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

        write!(writer, "4,")?;
        self.exposure.to_code(&mut writer)?;
        writeln!(writer, ",{},", self.points.len() - 1)?;

        for &(ref x, ref y) in self.points.iter() {
            x.to_code(&mut writer)?;
            write!(writer, ",")?;
            y.to_code(&mut writer)?;
            writeln!(writer, ",")?;
        }
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for PolygonPrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
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
        write!(writer, "5,")?;
        self.exposure.to_code(&mut writer)?;
        write!(writer, ",{},", self.vertices)?;
        self.center.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.diameter.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

/// The moiré primitive is a cross hair centered on concentric rings (annuli).
/// Exposure is always on.
#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for MoirePrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
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
        write!(writer, "6,")?;
        self.center.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.diameter.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.ring_thickness.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.gap.to_code(&mut writer)?;
        write!(writer, ",{},", self.max_rings)?;
        self.cross_hair_thickness.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.cross_hair_length.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

/// The thermal primitive is a ring (annulus) interrupted by four gaps.
/// Exposure is always on.
#[derive(Debug, PartialEq)]
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

impl<W: Write> GerberCode<W> for ThermalPrimitive {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        // Decimal invariants
        if self.inner_diameter.is_negative() {
            return Err(GerberError::RangeError("Inner diameter of a thermal may not be negative".into()));
        }
        write!(writer, "7,")?;
        self.center.0.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.center.1.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.outer_diameter.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.inner_diameter.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.gap.to_code(&mut writer)?;
        write!(writer, ",")?;
        self.angle.to_code(&mut writer)?;
        write!(writer, "*")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDefinition {
    number: u32,
    expression: String,
}

impl<W: Write> GerberCode<W> for VariableDefinition {
    fn to_code(&self, mut writer: &mut W) -> GerberResult<()> {
        write!(writer, "${}={}*", self.number, self.expression)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std::io::BufWriter;
    use super::*;
    use super::MacroDecimal::{Value, Variable};
    use ::GerberCode;

    macro_rules! assert_code {
        ($obj:expr, $expected:expr) => {
            let mut buf = BufWriter::new(Vec::new());
            $obj.to_code(&mut buf).expect("Could not generate Gerber code");
            let bytes = buf.into_inner().unwrap();
            let code = String::from_utf8(bytes).unwrap();
            assert_eq!(&code, $expected);
        }
    }

    #[test]
    fn test_circle_primitive_codegen() {
        let with_angle = CirclePrimitive {
            exposure: true,
            diameter: Value(1.5),
            center: (Value(0.), Value(0.)),
            angle: Some(Value(0.)),
        };
        assert_code!(with_angle, "1,1,1.5,0,0,0*");
        let no_angle = CirclePrimitive {
            exposure: false,
            diameter: Value(99.9),
            center: (Value(1.1), Value(2.2)),
            angle: None,
        };
        assert_code!(no_angle, "1,0,99.9,1.1,2.2*");
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
        assert_code!(line, "20,1,0.9,0,0.45,12,0.45,0*");
    }

    #[test]
    fn test_center_line_primitive_codegen() {
        let line = CenterLinePrimitive {
            exposure: true,
            dimensions: (Value(6.8), Value(1.2)),
            center: (Value(3.4), Value(0.6)),
            angle: Value(30.0),
        };
        assert_code!(line, "21,1,6.8,1.2,3.4,0.6,30*");
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
        assert_code!(line, "4,1,4,\n0.1,0.1,\n0.5,0.1,\n0.5,0.5,\n0.1,0.5,\n0.1,0.1,\n0*");
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
        assert_code!(line, "5,1,8,1.5,2,8,0*");
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
        assert_code!(line, "6,0,0,5,0.5,0.5,2,0.1,6,0*");
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
        assert_code!(line, "7,0,0,8,6.5,1,45*");
    }

    #[test]
    fn test_aperture_macro_codegen() {
        let am = ApertureMacro::new("CRAZY").add_content(
            MacroContent::Thermal(
                ThermalPrimitive {
                    center: (Value(0.0), Value(0.0)),
                    outer_diameter: Value(0.08),
                    inner_diameter: Value(0.055),
                    gap: Value(0.0125),
                    angle: Value(45.0),
                }
            )
        ).add_content(
            MacroContent::Moire(
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
        assert_code!(am, "AMCRAZY*\n7,0,0,0.08,0.055,0.0125,45*\n6,0,0,0.125,0.01,0.01,3,0.003,0.15,0*");
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
        assert_code!(line, "20,1,$0,$1,0.45,12,$2,$3*");
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
        let comment = MacroContent::Comment("hello world".to_string());
        assert_code!(comment, "0 hello world*");
    }

    #[test]
    fn test_variable_definition_codegen() {
        let var = VariableDefinition {
            number: 17,
            expression: "$40+2".to_string(),
        };
        assert_code!(var, "$17=$40+2*");
    }

    #[test]
    fn test_macrocontent_from_into() {
        let a = MacroContent::Comment("hello".into());
        let b: MacroContent = "hello".to_string().into();
        let c: MacroContent = "hello".into();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }
}
