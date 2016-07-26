trait GerberCode {
    fn to_code(&self) -> String;
}

#[derive(Debug)]
enum Command {
    FunctionCode(FunctionCode),
    ExtendedCode(ExtendedCode),
}

#[derive(Debug)]
enum FunctionCode {
    DCode(DCode),
    GCode(GCode),
    MCode(MCode),
}

#[derive(Debug)]
enum ExtendedCode {
    FS,
    MO,
    AD,
    AM,
    SR,
    LP,
    TF,
    TA,
    TD,
}

#[derive(Debug)]
enum DCode {
    Operation,
    SelectAperture,
}

#[derive(Debug)]
enum InterpolationMode {
    Linear,
    ClockwiseCircular,
    CounterclockwiseCircular,
}

#[derive(Debug)]
enum GCode {
    InterpolationMode(InterpolationMode),
    RegionMode,
    QuadrantMode,
    Comment(String),
}

#[derive(Debug)]
enum MCode {
    EndOfFile,
}

impl GerberCode for GCode {
    fn to_code(&self) -> String {
        match self {
            &GCode::InterpolationMode(ref mode) => {
                match *mode {
                    InterpolationMode::Linear => "G01*".to_string(),
                    InterpolationMode::ClockwiseCircular => "G02*".to_string(),
                    InterpolationMode::CounterclockwiseCircular => "G03*".to_string(),
                }
            },
            &GCode::RegionMode => format!("TODO RegionMode"),
            &GCode::QuadrantMode => format!("TODO QuadrantMode"),
            &GCode::Comment(ref comment) => format!("G04 {} *", comment),
        }
    }
}

impl<T: GerberCode> GerberCode for Vec<T> {
    fn to_code(&self) -> String {
        self.iter()
            .map(|g| g.to_code())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Command, FunctionCode, DCode, GCode, InterpolationMode};
    use super::GerberCode;

    #[test]
    fn test_gcode_debug() {
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment("test".to_string())));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment(\"test\")))");
    }

    #[test]
    fn test_to_code() {
        let comment = GCode::Comment("testcomment".to_string());
        assert_eq!(comment.to_code(), "G04 testcomment *".to_string());
    }

    #[test]
    fn test_vec_to_code() {
        let mut v = Vec::new();
        v.push(GCode::Comment("comment 1".to_string()));
        v.push(GCode::Comment("another one".to_string()));
        assert_eq!(v.to_code(), "G04 comment 1 *\nG04 another one *".to_string());
    }

    #[test]
    fn test_interpolation_mode() {
        let mut commands = Vec::new();
        let c1 = GCode::InterpolationMode(InterpolationMode::Linear);
        let c2 = GCode::InterpolationMode(InterpolationMode::ClockwiseCircular);
        let c3 = GCode::InterpolationMode(InterpolationMode::CounterclockwiseCircular);
        commands.push(c1);
        commands.push(c2);
        commands.push(c3);
        assert_eq!(commands.to_code(), "G01*\nG02*\nG03*".to_string());
    }

}
