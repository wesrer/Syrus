use super::message_traits::{Decode, Encode};
use crate::block_exchange_protocol::{Header, MessageCompression, MessageType};
use crate::errors::{Errors, InvalidMessageErrors};
use bytes::{Buf, BufMut, BytesMut};
use compress::lz4;
use prost::Message;

// impl Decode for MessageType {
//     fn decode_from(buffer: &BytesMut) {}
// }

impl Decode for Header {
    fn decode_from(mut buffer: &mut BytesMut) -> Result<Header, Errors> {
        let header_len = read_header_length(&mut buffer);

        let header = buffer.split_to(header_len as usize);

        match Header::decode(header) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(Errors::DecodeError(e)),
        }
    }
}

impl Header {
    fn message_type(&self) -> Result<MessageType, Errors> {
        match MessageType::from_i32(self.r#type) {
            Some(x) => Ok(x),
            None => Err(Errors::InvalidMessageErrors(
                InvalidMessageErrors::invalid_message_type(),
            )),
        }
    }

    fn message_compression(&self) -> Result<MessageCompression, Errors> {
        match MessageCompression::from_i32(self.compression) {
            Some(x) => Ok(x),
            None => Err(Errors::InvalidMessageErrors(
                InvalidMessageErrors::invalid_message_compression(),
            )),
        }
    }
}

fn read_header_length(buffer: &mut BytesMut) -> u16 {
    buffer.split_to(2).get_u16()
}
