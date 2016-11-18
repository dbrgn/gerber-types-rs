//! Error types used in the gerber-types library.

quick_error! {
    #[derive(Debug)]
    pub enum GerberError {
        /// Conversion between two types failed
        ConversionError(msg: String) {}
        /// Bad coordinate format
        CoordinateFormatError(msg: String) {}
    }
}
