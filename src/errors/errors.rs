use prost::{DecodeError, EncodeError};

#[derive(Debug)]
pub enum Errors {
    InvalidMessageError(InvalidMessageError),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
}

#[derive(Debug)]
pub enum InvalidMessageError {
    InvalidHello(String),
    IncorrectLengthSpecified(String),
    InvalidVersionSyntax(String),
    InvalidMessageType(String),
    InvalidMessageCompression(String),
}

impl InvalidMessageError {
    pub fn invalid_hello() -> Self {
        Self::InvalidHello(
            "Incorrect Magic Number at the beginning of message. Expected Hello".to_string(),
        )
    }

    pub fn incorrect_length(msg_type: String) -> Self {
        Self::IncorrectLengthSpecified(
            format!("Length Specified by {} message doesn't match actual message length. Cannot establish connection.", msg_type)
        )
    }

    pub fn invalid_version() -> Self {
        Self::InvalidVersionSyntax(
            "Client version provided does not semantic versioning standards. Cannot connect to client."
                .to_string(),
        )
    }

    pub fn invalid_message_type() -> Self {
        Self::InvalidMessageType(
            "Message Type not recognized. Could not decode message.".to_string(),
        )
    }

    pub fn invalid_message_compression() -> Self {
        Self::InvalidMessageCompression(
            "Message Compression type not recognized. Could not decode message.".to_string(),
        )
    }
}
