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
    Comment,
}

#[derive(Debug)]
enum MCode {
    EndOfFile,
}

#[cfg(test)]
mod test {
    use super::{Command, FunctionCode, DCode, GCode};

    #[test]
    fn test_gcode_debug() {
        let c = Command::FunctionCode(FunctionCode::GCode(GCode::Comment));
        let debug = format!("{:?}", c);
        assert_eq!(debug, "FunctionCode(GCode(Comment))");
    }

}
