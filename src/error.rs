use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("serialization error")]
    SerializationError(String),

    #[error("deserialization error")]
    DeserializationError(String),
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
