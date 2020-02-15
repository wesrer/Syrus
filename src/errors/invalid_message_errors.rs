use crate::errors::Errors;

#[derive(Debug, PartialEq)]
pub enum InvalidMessageError {
    InvalidHello(String),
    IncorrectLengthSpecified(String),
    InvalidVersionSyntax(String),
    InvalidMessageType(String),
    InvalidMessageCompression(String),
}

impl InvalidMessageError {
    pub fn invalid_hello() -> Errors {
        Errors::InvalidMessageError(InvalidMessageError::InvalidHello(
            "Incorrect Magic Number at the beginning of message. Expected Hello".to_string(),
        ))
    }

    pub fn incorrect_length(msg_type: String) -> Errors {
        Errors::InvalidMessageError(InvalidMessageError::IncorrectLengthSpecified(
            format!("Length Specified by {} message doesn't match actual message length. Cannot establish connection.", msg_type)
        ))
    }

    pub fn invalid_version() -> Errors {
        Errors::InvalidMessageError(InvalidMessageError::InvalidVersionSyntax(
            "Client version provided does not semantic versioning standards. Cannot connect to client."
                .to_string(),
        ))
    }

    pub fn invalid_message_type() -> Errors {
        Errors::InvalidMessageError(InvalidMessageError::InvalidMessageType(
            "Message Type not recognized. Could not decode message.".to_string(),
        ))
    }

    pub fn invalid_message_compression() -> Errors {
        Errors::InvalidMessageError(InvalidMessageError::InvalidMessageCompression(
            "Message Compression type not recognized. Could not decode message.".to_string(),
        ))
    }
}
