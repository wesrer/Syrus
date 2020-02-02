use super::{Decode, Encode, Utils};
use crate::block_exchange_protocol::{
    Close, ClusterConfig, DownloadProgress, Header, Index, IndexUpdate, MessageCompression,
    MessageType, Ping, Request, Response,
};
use crate::errors::{Errors, InvalidMessageError};
use bytes::{Buf, BufMut, BytesMut};
use compress::lz4;

// TODO: Assess whether we need this data structure, or if we can directly use Messages
//       It's currently here because I am unsure whether we need the header at a later
//       stage in the program evaluation
pub struct MessageContent {
    header: Header,
    message: Messages,
}

macro_rules! messages_enum {
    ($name:ident { $($variant:ident),* })   => (
        #[derive(Debug, PartialEq)]
        pub enum $name { $($variant($variant)),* }

        $(impl Decode for $variant {})*
    );
}

// Expands to:
// pub enum Messages {
//      ClusterConfig(ClusterConfig),
//      ...
// }
// Where the ClusterConfig inside the paren is the struct initialized by prost
#[macro_use]
messages_enum!(Messages {
    ClusterConfig,
    Index,
    IndexUpdate,
    Request,
    Response,
    DownloadProgress,
    Ping,
    Close
});

impl Decode for MessageContent {
    fn decode_from(buffer: &mut BytesMut) -> Result<Self, Errors> {
        let header = Header::decode_from(buffer)?;

        // NOTE: We are getting i32 because of the protocol specifications of always assuming we receive ints
        //       and not unsigneds, even though it doesn't make much sense to received signed values.
        // TODO: Assess whether we should verify if this is ever negative, which would indicate a corrupt block
        let msg_len = buffer.split_to(4).get_i32();

        // TODO: instead of sending post-auth message as the message type, generate strings from message types
        Self::verify_len(&buffer, msg_len as usize, "Post-Auth".to_string())?;

        let mut decompressed = decompress(buffer, header.message_compression()?);

        let decoded_message = MessageContent::from_buffer_and_message_type(
            &mut decompressed,
            header.message_type()?,
        )?;

        Ok(MessageContent {
            header: header,
            message: decoded_message,
        })
    }
}

fn decompress(buffer: &mut BytesMut, compression: MessageCompression) -> BytesMut {
    if let MessageCompression::Lz4 = compression {
        let mut decodebuf = Vec::new();
        let mut finalbuf = BytesMut::new();

        lz4::decode_block(&buffer, &mut decodebuf);
        finalbuf.put_slice(&decodebuf);

        finalbuf
    } else {
        // don't need to do anything if there is no compression detected

        // NOTE: Hacky solution to get a uniform API for both
        //       compressed cases and non-compressed cases
        buffer.clone()
    }
}

impl MessageContent {
    fn from_buffer_and_message_type(
        buffer: &mut BytesMut,
        message_type: MessageType,
    ) -> Result<Messages, Errors> {
        // NOTE: Cannot shorten this with a macro because the current version of Rust
        //       doesn't let you iterate over Enums. If this changes in future versions,
        //       we can get rid of the repitition here.
        Ok(match message_type {
            MessageType::ClusterConfig => {
                Messages::ClusterConfig(ClusterConfig::decode_from(buffer)?)
            }
            MessageType::Index => Messages::Index(Index::decode_from(buffer)?),
            MessageType::IndexUpdate => Messages::IndexUpdate(IndexUpdate::decode_from(buffer)?),
            MessageType::Request => Messages::Request(Request::decode_from(buffer)?),
            MessageType::Response => Messages::Response(Response::decode_from(buffer)?),
            MessageType::DownloadProgress => {
                Messages::DownloadProgress(DownloadProgress::decode_from(buffer)?)
            }
            MessageType::Ping => Messages::Ping(Ping::decode_from(buffer)?),
            MessageType::Close => Messages::Close(Close::decode_from(buffer)?),
        })
    }
}

impl Utils for MessageContent {}

// Implement the decode trait for all the message types
