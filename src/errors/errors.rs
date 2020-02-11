use crate::errors::invalid_message_errors::InvalidMessageError;
use prost::{DecodeError, EncodeError};

#[derive(Debug, PartialEq)]
pub enum Errors {
    InvalidMessageError(InvalidMessageError),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
    InternalError(InternalError),
}

#[derive(Debug, PartialEq)]
pub enum InternalError {
    DeviceIdGenerationError(String),
    CharacterFetchError(String),
}

impl InternalError {
    // TODO: Maybe refactor the success type to use generics
    pub fn device_id_generation_error(q: char, alphabet: &str) -> Result<char, Errors> {
        Err(Errors::InternalError(
            InternalError::DeviceIdGenerationError(format!(
                "Char {} is not valid in alphabet {}",
                q, alphabet
            )),
        ))
    }

    pub fn character_fetch_error(q: usize, alphabet: &str) -> Result<char, Errors> {
        Err(Errors::InternalError(InternalError::CharacterFetchError(
            format!("Could not fetch character {} from alphabet {}", q, alphabet),
        )))
    }
}
