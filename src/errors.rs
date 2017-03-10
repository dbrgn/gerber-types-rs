//! Error types used in the gerber-types library.

quick_error! {
    #[derive(Debug)]
    pub enum GerberError {
        /// Conversion between two types failed
        ConversionError(msg: String) {}
        /// Bad coordinate format
        CoordinateFormatError(msg: String) {}
        /// A value is out of range
        RangeError(msg: String) {}
        /// Required data is missing
        MissingDataError(msg: String) {}
    }
}

pub type GerberResult<T> = Result<T, GerberError>;
