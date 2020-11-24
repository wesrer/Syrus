use crate::errors::{Errors, InvalidMessageError};
use bytes::BytesMut;
use prost::Message;

pub trait Encode {
    fn encode_to_bytes(&self) -> Result<BytesMut, Errors>;
}

pub trait Decode {
    fn decode_from(buffer: &mut BytesMut) -> Result<Self, Errors>
    where
        Self: std::marker::Sized + Message + std::default::Default,
    {
        // Try to decode with the prost decode function given for the
        // implemented struct, and propagate errors upwards
        Self::decode(buffer).map_err(|e| Errors::DecodeError(e))
    }
}

pub trait Utils {
    fn verify_len(buffer: &BytesMut, size: usize, msg_type: String) -> Result<(), Errors> {
        if buffer.len() != size {
            return Err(InvalidMessageError::incorrect_length(msg_type));
        }

        Ok(())
    }
}
