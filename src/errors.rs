//! Error types used in the gerber-types library.

use std::io::Error as IoError;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum GerberError {
        /// Conversion between two types failed
        ConversionError(msg: String) {}
        /// Bad coordinate format
        CoordinateFormatError(msg: String) {}
        /// A value is out of range
        RangeError(msg: String) {}
        /// Required data is missing
        MissingDataError(msg: String) {}
        /// I/O error during code generation
        IoError(err: IoError) {
            cause(err)
            from()
        }
    }
}

pub type GerberResult<T> = Result<T, GerberError>;
