use prost::{DecodeError, EncodeError};

#[derive(Debug)]
pub enum InvalidMessageErrors {
    InvalidHello(String),
    IncorrectLengthSpecified(String),
}

#[derive(Debug)]
pub enum Errors {
    InvalidMessageErrors(InvalidMessageErrors),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
}
