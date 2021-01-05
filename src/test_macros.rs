/// Assert that serializing the object generates
/// the specified gerber code.
macro_rules! assert_code {
    ($obj:expr, $expected:expr) => {
        let mut buf = BufWriter::new(Vec::new());
        $obj.serialize(&mut buf)
            .expect("Could not generate Gerber code");
        let bytes = buf.into_inner().unwrap();
        let code = String::from_utf8(bytes).unwrap();
        assert_eq!(&code, $expected);
    };
}

/// Assert that partially serializing the object generates
/// the specified gerber code.
macro_rules! assert_partial_code {
    ($obj:expr, $expected:expr) => {
        let mut buf = BufWriter::new(Vec::new());
        $obj.serialize_partial(&mut buf)
            .expect("Could not generate Gerber code");
        let bytes = buf.into_inner().unwrap();
        let code = String::from_utf8(bytes).unwrap();
        assert_eq!(&code, $expected);
    };
}
