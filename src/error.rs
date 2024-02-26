//! ### Error
//! A module for the error type used in the library. It is a simple enum with a variant for each
//! error that can occur in the library. It uses `thiserror` internally.

use super::serializer::Delimiter;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not get the last bit from the data.")]
    NoBit,

    #[error("could not get the last byte from the data.")]
    NoByte,

    #[error("tried to get {0} bytes from the data of length {1}.")]
    NLargerThanLength(usize, usize),

    #[error("could not serialize the value: {0}")]
    SerializationError(String),

    #[error("could not deserialize the value: {0}")]
    DeserializationError(String),

    #[error("calls to {0} are not supported")]
    UnsupportedCall(String),

    #[error("unexpected end of file")]
    UnexpectedEOF,

    #[error("invalid type size")]
    InvalidTypeSize,

    #[error("type conversion error")]
    ConversionError,

    #[error("expected delimiter {0}")]
    ExpectedDelimiter(Delimiter),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::SerializationError(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
