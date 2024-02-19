#[derive(thiserror::Error, Debug)]
pub enum Error {
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

    #[error("expected string delimiter")]
    ExpectedStringDelimiter,

    #[error("expected byte delimiter")]
    ExpectedByteDelimiter,

    #[error("expected unit")]
    ExpectedUnit,

    #[error("expected enum delimiter")]
    ExpectedEnumDelimiter,

    #[error("expected seq delimiter")]
    ExpectedSeqDelimiter,

    #[error("expected seq value delimiter")]
    ExpectedSeqValueDelimiter,

    #[error("expected map delimiter")]
    ExpectedMapDelimiter,
    #[error("expected map key delimiter")]
    ExpectedMapKeyDelimiter,
        #[error("expected map value separator")]
    ExpectedMapValueSeparator,
    #[error("expected map value delimiter")]
    ExpectedMapValueDelimiter,
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
