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
enum GCode {
    InterpolationMode,
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
            &GCode::InterpolationMode => format!("TODO InterpolationMode"),
            &GCode::RegionMode => format!("TODO RegionMode"),
            &GCode::QuadrantMode => format!("TODO QuadrantMode"),
            &GCode::Comment(ref comment) => format!("G04 {} *\n", comment),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Command, FunctionCode, DCode, GCode};
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
        assert_eq!(comment.to_code(), "G04 testcomment *\n".to_string());
    }

}
