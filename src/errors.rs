//! Error types used in the gerber-types library.

quick_error! {
    #[derive(Debug)]
    pub enum GerberError {
        /// Bad coordinate format
        CoordinateFormatError(err: String) {}
    }
}
