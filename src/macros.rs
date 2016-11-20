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
}

impl GerberCode for Primitive {
    fn to_code(&self) -> GerberResult<String> {
        let code = match *self {
            Primitive::Comment(ref s) => s.clone(),
            Primitive::Circle(ref c) => try!(c.to_code()),
            Primitive::VectorLine(ref vl) => try!(vl.to_code()),
            Primitive::CenterLine(ref cl) => try!(cl.to_code()),
            Primitive::Outline(ref o) => try!(o.to_code()),
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
}
