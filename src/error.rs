use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("end of file")]
    EOF,

    #[error("not supported: {0}")]
    NotSupported(String),

    #[error("unexpected end of file")]
    UnexpectedEOF,

    #[error("invalid type size")]
    InvalidTypeSize,

    #[error("type conversion error")]
    ConversionError,

    #[error("expected null value, found non-null value")]
    ExpectedNull,

    #[error("expected a dagger seperator, found non-dagger value")]
    ExpectedDagger,

    #[error("expected a pipe, found non-pipe value")]
    ExpectedPipe,

    #[error("expected a enum, found non-enum value")]
    ExpectedEnum,

    #[error("expected a u32, found non-32 value")]
    ExpectedU32,

    #[error("expected sequence start, found else")]
    ExpectedSequenceStart,

    #[error("expected sequence end, found else")]
    ExpectedSequenceEnd,

    #[error("expected map start, found else")]
    ExpectedMapStart,

    #[error("expected map end, found else")]
    ExpectedMapEnd,

    #[error("unexpected none value: {0}")]
    UnexpectedNone(String),
}

impl serde::ser::Error for CustomError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        CustomError::SerializationError(msg.to_string())
    }
}

impl serde::de::Error for CustomError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        CustomError::DeserializationError(msg.to_string())
    }
}
