use crate::{
    block_exchange_protocol::{Header, MessageCompression, MessageType},
    core::messages::{Decode, Encode},
    errors::{Errors, InvalidMessageError},
};

use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

impl Decode for Header {
    fn decode_from(mut buffer: &mut BytesMut) -> Result<Header, Errors> {
        let header_len = read_header_length(&mut buffer);

        let header = buffer.split_to(header_len as usize);

        Header::decode(header).map_err(|e| Errors::DecodeError(e))
    }
}

impl Encode for Header {
    fn encode_to_bytes(&self) -> Result<BytesMut, Errors> {
        let mut finalbuf = BytesMut::new();
        let mut encodebuf: Vec<u8> = Vec::new();

        match self.encode(&mut encodebuf) {
            Ok(_) => {
                finalbuf.put_i16(encodebuf.len() as i16);
                finalbuf.put_slice(&encodebuf);
                Ok(finalbuf)
            }
            Err(e) => Err(Errors::EncodeError(e)),
        }
    }
}

impl Header {
    pub fn message_type(&self) -> Result<MessageType, Errors> {
        MessageType::from_i32(self.r#type).ok_or(InvalidMessageError::invalid_message_type())
    }

    pub fn message_compression(&self) -> Result<MessageCompression, Errors> {
        MessageCompression::from_i32(self.compression)
            .ok_or(InvalidMessageError::invalid_message_compression())
    }
}

fn read_header_length(buffer: &mut BytesMut) -> u16 {
    // Splits the bytes into at index 2
    // Afterwards, buffer contains the bytes [2, len] and this returns the rest
    buffer.split_to(2).get_u16()
}
