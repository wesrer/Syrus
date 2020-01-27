use crate::errors::Errors;
use bytes::{Buf, BufMut, BytesMut};

pub trait Encode {
    fn encode_to(msg: Self) -> Result<BytesMut, Errors>;
}

pub trait Decode {
    fn decode_from(buffer: &mut BytesMut) -> Result<Self, Errors>
    where
        Self: std::marker::Sized;
}
