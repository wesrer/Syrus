use prost::{DecodeError, EncodeError};

pub enum InvalidMessageErrors {
    InvalidHello(String),
    IncorrectLengthSpecified(String),
}

pub enum Errors {
    InvalidMessageErrors(InvalidMessageErrors),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
}
