//! Aperture Macros.

use ::{GerberCode, GerberResult};

#[derive(Debug)]
pub struct ApertureMacro {
    pub name: String,
}

#[derive(Debug)]
pub enum Primitive {
    Comment(String),
    Circle(CirclePrimitive),
}

#[derive(Debug)]
pub struct CirclePrimitive {
    /// Exposure off/on
    pub exposure: bool,

    /// Diameter, a decimal >= 0
    pub diameter: f64,

    /// X and Y coordinates of center position, a decimal
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
        code.push_str(if self.exposure { "1" } else { "0" });
        code.push_str(&format!(",{},{},{}", self.diameter, self.center.0, self.center.1));
        if let Some(a) = self.angle {
            code.push_str(&format!(",{}", a));
        }
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
        assert_eq!(with_angle.to_code().unwrap(), "1,1,1.5,0,0,0".to_string());
        let no_angle = CirclePrimitive {
            exposure: false,
            diameter: 99.9,
            center: (1.1, 2.2),
            angle: None,
        };
        assert_eq!(no_angle.to_code().unwrap(), "1,0,99.9,1.1,2.2".to_string());
    }
}