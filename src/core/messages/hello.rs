use crate::{
    block_exchange_protocol::Hello,
    core::messages::{Decode, Encode, Utils},
    errors::{Errors, InvalidMessageError},
    globals,
};
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

impl Encode for Hello {
    fn encode_to_bytes(&self) -> Result<BytesMut, Errors> {
        // TODO: Check whether the versioning follow semantic versioning standards

        let mut finalbuf = BytesMut::new();
        let mut encodebuf: Vec<u8> = Vec::new();

        // encode the message into encode buffer and match on the returned result
        match self.encode(&mut encodebuf) {
            Ok(_) => {
                // Add the magic number for the Hello message to the beginning
                // of the buffer
                finalbuf.put_i32(globals::MAGIC_NUMBER_HELLO_MESSAGE);

                // Add the size of the encoding to the buffer next
                finalbuf.put_i16(encodebuf.len() as i16);

                // read bytes from the encoding and insert them into the return buffer
                finalbuf.put_slice(&encodebuf);

                Ok(finalbuf)
            }
            Err(e) => Err(Errors::EncodeError(e)),
        }
    }
}

impl Decode for Hello {
    fn decode_from(buffer: &mut BytesMut) -> Result<Hello, Errors> {
        // since the buffer reads in bytes, we have to divide things up into
        // byte indexes ->
        //    32 bit is 4 bytes -> Index 0 - 3
        //    16 bit is 2 bytes -> Index 0 - 1
        let magic_num = buffer.split_to(4).get_i32();
        let size = buffer.split_to(2).get_i16();

        // Guard clauses to throw errors if packet is unreliable
        is_hello(magic_num)?;
        Hello::verify_len(&buffer, size as usize, "Hello".to_string())?;

        // Use the prost generated decode now that we have verified that the
        // packet doesn't have obvious easy to catch errors.
        Hello::decode(buffer).map_err(|e| Errors::DecodeError(e))
    }
}

impl Utils for Hello {}

fn is_hello(magic_number: i32) -> Result<(), Errors> {
    if magic_number != globals::MAGIC_NUMBER_HELLO_MESSAGE {
        return Err(InvalidMessageError::invalid_hello());
    }
    Ok(())
}
