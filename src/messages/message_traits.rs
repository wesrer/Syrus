use crate::block_exchange_protocol::Header;
use crate::errors::{Errors, InvalidMessageError};
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

pub trait Encode {
    fn encode_to_bytes(&self) -> Result<BytesMut, Errors>;
}

pub trait Decode {
    fn decode_from(buffer: &mut BytesMut) -> Result<Self, Errors>
    where
        Self: std::marker::Sized + Message + std::default::Default,
    {
        match Self::decode(buffer) {
            Ok(x) => Ok(x),
            Err(e) => Err(Errors::DecodeError(e)),
        }
    }
}

pub trait Utils {
    fn verify_len(buffer: &BytesMut, size: usize, msg_type: String) -> Result<(), Errors> {
        if buffer.len() != size {
            return Err(Errors::InvalidMessageError(
                InvalidMessageError::incorrect_length(msg_type),
            ));
        }

        Ok(())
    }
}
