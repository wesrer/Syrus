use crate::errors::Errors;
use bytes::{Buf, BufMut, BytesMut};

pub trait EncodeTo {
    fn encode_to(msg: Self) -> Result<BytesMut, Errors>;
}

pub trait DecodeFrom {
    fn decode_from(buffer: BytesMut) -> Result<Self, Errors>
    where
        Self: std::marker::Sized;
}
