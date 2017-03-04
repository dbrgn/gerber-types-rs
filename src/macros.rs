//! Aperture Macros.

use ::{GerberCode, GerberError, GerberResult};

#[derive(Debug)]
pub struct ApertureMacro {
    pub name: String,
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
            Primitive::Comment(ref s) => s.clone(),
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
    pub diameter: f64,

    /// X and Y coordinates of center position, decimals
    pub center: (f64, f64),

    /// Rotation angle.
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    ///
    /// The rotation modifier is optional. The default is no rotation. (We
    /// recommend always to set the angle explicitly.
    pub angle: Option<f64>,
}

impl GerberCode for CirclePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let mut code = "1,".to_string();
        code.push_str(&try!(self.exposure.to_code()));
        code.push_str(&format!(",{},{},{}", self.diameter, self.center.0, self.center.1));
        if let Some(a) = self.angle {
            code.push_str(&format!(",{}", a));
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
    pub width: f64,

    /// X and Y coordinates of start point, decimals
    pub start: (f64, f64),

    /// X and Y coordinates of end point, decimals
    pub end: (f64, f64),

    /// Rotation angle of the vector line primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    pub angle: f64,
}

impl GerberCode for VectorLinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = format!(
            "20,{},{},{},{},{},{},{}*",
            try!(self.exposure.to_code()),
            self.width,
            self.start.0, self.start.1, self.end.0, self.end.1,
            self.angle
        );
        Ok(code)
    }
}

#[derive(Debug)]
pub struct CenterLinePrimitive {
    /// Exposure off/on (0/1)
    pub exposure: bool,

    /// Rectangle dimensions (width/height)
    pub dimensions: (f64, f64),

    /// X and Y coordinates of center point, decimals
    pub center: (f64, f64),

    /// Rotation angle
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. (0, 0) point
    /// of macro coordinates.
    pub angle: f64,
}

impl GerberCode for CenterLinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = format!(
            "21,{},{},{},{},{},{}*",
            try!(self.exposure.to_code()),
            self.dimensions.0, self.dimensions.1,
            self.center.0, self.center.1,
            self.angle
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
    pub points: Vec<(f64, f64)>,

    /// Rotation angle of the outline primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    pub angle: f64,
}

impl GerberCode for OutlinePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Points invariants
        if self.points.len() < 2 {
            return Err(GerberError::RangeError("There must be at least 1 subsequent point in an outline".into()));
        }
        if self.points.len() > 5001 {
            return Err(GerberError::RangeError("The maximum number of subsequent points in an outline is 5000".into()));
        }
        if self.points[0] != self.points[self.points.len() - 1] {
            return Err(GerberError::RangeError("The maximum number of subsequent points n is 5000".into()));
        }

        let mut code = format!("4,{},{},\n", try!(self.exposure.to_code()), self.points.len() - 1);
        code.push_str(
            &self.points.iter()
                        .map(|&(x, y)| format!("{},{},", x, y))
                        .collect::<Vec<String>>()
                        .join("\n")
        );
        code.push_str(&format!("\n{}*", self.angle));
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
    pub center: (f64, f64),

    /// Diameter of the circumscribed circle, a decimal >= 0
    pub diameter: f64,

    /// Rotation angle of the polygon primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates. The first vertex is on the positive X-axis
    /// through the center point when the rotation angle is zero.
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: f64,
}

impl GerberCode for PolygonPrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Vertice count invariants
        if self.vertices < 3 {
            return Err(GerberError::RangeError("There must be at least 3 vertices in a polygon".into()));
        }
        if self.vertices > 12 {
            return Err(GerberError::RangeError("The maximum number of vertices in a polygon is 12".into()));
        }
        if self.diameter < 0.0 {
            return Err(GerberError::RangeError("The diameter must not be negative".into()));
        }
        let code = format!(
            "5,{},{},{},{},{},{}*",
            try!(self.exposure.to_code()),
            self.vertices,
            self.center.0, self.center.1,
            self.diameter,
            self.angle
        );
        Ok(code)
    }
}

/// The moiré primitive is a cross hair centered on concentric rings (annuli).
/// Exposure is always on.
#[derive(Debug)]
pub struct MoirePrimitive {
    /// X and Y coordinates of center point, decimals
    pub center: (f64, f64),

    /// Outer diameter of outer concentric ring, a decimal >= 0
    pub diameter: f64,

    /// Ring thickness, a decimal >= 0
    pub ring_thickness: f64,

    /// Gap between rings, a decimal >= 0
    pub gap: f64,

    /// Maximum number of rings
    pub max_rings: u32,

    /// Cross hair thickness, a decimal >= 0
    pub cross_hair_thickness: f64,

    /// Cross hair length, a decimal >= 0
    pub cross_hair_length: f64,

    /// Rotation angle of the moiré primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates.
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: f64,
}

impl GerberCode for MoirePrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Decimal invariants
        if self.diameter < 0.0 {
            return Err(GerberError::RangeError("Outer diameter of a moiré may not be negative".into()));
        }
        if self.ring_thickness < 0.0 {
            return Err(GerberError::RangeError("Ring thickness of a moiré may not be negative".into()));
        }
        if self.gap < 0.0 {
            return Err(GerberError::RangeError("Gap of a moiré may not be negative".into()));
        }
        if self.cross_hair_thickness < 0.0 {
            return Err(GerberError::RangeError("Cross hair thickness of a moiré may not be negative".into()));
        }
        if self.cross_hair_length < 0.0 {
            return Err(GerberError::RangeError("Cross hair length of a moiré may not be negative".into()));
        }
        let code = format!(
            "6,{},{},{},{},{},{},{},{},{}*",
            self.center.0, self.center.1,
            self.diameter,
            self.ring_thickness,
            self.gap,
            self.max_rings,
            self.cross_hair_thickness, self.cross_hair_length,
            self.angle
        );
        Ok(code)
    }
}

/// The thermal primitive is a ring (annulus) interrupted by four gaps.
/// Exposure is always on.
#[derive(Debug)]
pub struct ThermalPrimitive {
    /// X and Y coordinates of center point, decimals
    pub center: (f64, f64),

    /// Outer diameter, a decimal > inner diameter
    pub outer_diameter: f64,

    /// Inner diameter, a decimal >= 0
    pub inner_diameter: f64,

    /// Gap thickness, a decimal < (outer diameter) / sqrt(2)
    pub gap: f64,

    /// Rotation angle of the thermal primitive
    ///
    /// The rotation angle is specified by a decimal, in degrees. The primitive
    /// is rotated around the origin of the macro definition, i.e. the (0, 0)
    /// point of macro coordinates. The gaps are on the X and Y axes through
    /// the center when the rotation angle is zero
    ///
    /// Note: Rotation is only allowed if the primitive center point coincides
    /// with the origin of the macro definition.
    pub angle: f64,
}

impl GerberCode for ThermalPrimitive {
    fn to_code(&self) -> GerberResult<String> {
        // Decimal invariants
        if self.inner_diameter < 0.0 {
            return Err(GerberError::RangeError("Inner diameter of a thermal may not be negative".into()));
        }
        if self.outer_diameter <= self.inner_diameter {
            return Err(GerberError::RangeError("Outer diameter of a thermal must be larger than inner diameter".into()));
        }
        if self.gap > (self.outer_diameter / 2f64.sqrt()) {
            return Err(GerberError::RangeError("Gap of a thermal must be smaller than outer_diameter/sqrt(2)".into()));
        }
        let code = format!(
            "7,{},{},{},{},{},{}*",
            self.center.0, self.center.1,
            self.outer_diameter,
            self.inner_diameter,
            self.gap,
            self.angle
        );
        Ok(code)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use ::GerberCode;

    #[test]
    fn test_circle_primitive_codegen() {
        let with_angle = CirclePrimitive {
            exposure: true,
            diameter: 1.5,
            center: (0., 0.),
            angle: Some(0.),
        };
        assert_eq!(with_angle.to_code().unwrap(), "1,1,1.5,0,0,0*".to_string());
        let no_angle = CirclePrimitive {
            exposure: false,
            diameter: 99.9,
            center: (1.1, 2.2),
            angle: None,
        };
        assert_eq!(no_angle.to_code().unwrap(), "1,0,99.9,1.1,2.2*".to_string());
    }

    #[test]
    fn test_vector_line_primitive_codegen() {
        let line = VectorLinePrimitive {
            exposure: true,
            width: 0.9,
            start: (0., 0.45),
            end: (12., 0.45),
            angle: 0.,
        };
        assert_eq!(line.to_code().unwrap(), "20,1,0.9,0,0.45,12,0.45,0*".to_string());
    }

    #[test]
    fn test_center_line_primitive_codegen() {
        let line = CenterLinePrimitive {
            exposure: true,
            dimensions: (6.8, 1.2),
            center: (3.4, 0.6),
            angle: 30.0,
        };
        assert_eq!(line.to_code().unwrap(), "21,1,6.8,1.2,3.4,0.6,30*".to_string());
    }

    #[test]
    fn test_outline_primitive_codegen() {
        let line = OutlinePrimitive {
            exposure: true,
            points: vec![
                (0.1, 0.1),
                (0.5, 0.1),
                (0.5, 0.5),
                (0.1, 0.5),
                (0.1, 0.1),
            ],
            angle: 0.0,
        };
        assert_eq!(line.to_code().unwrap(), "4,1,4,\n0.1,0.1,\n0.5,0.1,\n0.5,0.5,\n0.1,0.5,\n0.1,0.1,\n0*".to_string());
    }

    #[test]
    fn test_polygon_primitive_codegen() {
        let line = PolygonPrimitive {
            exposure: true,
            vertices: 8,
            center: (1.5, 2.0),
            diameter: 8.0,
            angle: 0.0,
        };
        assert_eq!(line.to_code().unwrap(), "5,1,8,1.5,2,8,0*".to_string());
    }

    #[test]
    fn test_moire_primitive_codegen() {
        let line = MoirePrimitive {
            center: (0.0, 0.0),
            diameter: 5.0,
            ring_thickness: 0.5,
            gap: 0.5,
            max_rings: 2,
            cross_hair_thickness: 0.1,
            cross_hair_length: 6.0,
            angle: 0.0,
        };
        assert_eq!(line.to_code().unwrap(), "6,0,0,5,0.5,0.5,2,0.1,6,0*".to_string());
    }

    #[test]
    fn test_thermal_primitive_codegen() {
        let line = ThermalPrimitive {
            center: (0.0, 0.0),
            outer_diameter: 8.0,
            inner_diameter: 6.5,
            gap: 1.0,
            angle: 45.0,
        };
        assert_eq!(line.to_code().unwrap(), "7,0,0,8,6.5,1,45*".to_string());
    }
}
