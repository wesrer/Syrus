use prost::{DecodeError, EncodeError};

#[derive(Debug)]
pub enum InvalidMessageErrors {
    InvalidHello(String),
    IncorrectLengthSpecified(String),
    InvalidVersionSyntax(String),
}

impl InvalidMessageErrors {
    pub fn invalid_hello() -> Self {
        Self::InvalidHello(
            "Incorrect Magic Number at the beginning of message. Expected Hello".to_string(),
        )
    }

    pub fn incorrect_length() -> Self {
        Self::IncorrectLengthSpecified(
            "Length Specified by hello message doesn't match actual message length. Cannot establish connection."
                .to_string(),
        )
    }

    pub fn invalid_version() -> Self {
        Self::InvalidVersionSyntax(
            "Client version provided does not semantic versioning standards. Cannot connect to client."
                .to_string(),
        )
    }
}

#[derive(Debug)]
pub enum Errors {
    InvalidMessageErrors(InvalidMessageErrors),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
}
