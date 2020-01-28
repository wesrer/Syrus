use crate::errors::{Errors, InvalidMessageErrors};
use bytes::{Buf, BufMut, BytesMut};

pub trait Encode {
    fn encode_to_bytes(&self) -> Result<BytesMut, Errors>;
}

pub trait Decode {
    fn decode_from(buffer: &mut BytesMut) -> Result<Self, Errors>
    where
        Self: std::marker::Sized;
}

pub trait Utils {
    fn verify_len(buffer: &BytesMut, size: usize, msg_type: String) -> Result<(), Errors> {
        if buffer.len() != size {
            return Err(Errors::InvalidMessageErrors(
                InvalidMessageErrors::incorrect_length(msg_type),
            ));
        }

        Ok(())
    }
}
