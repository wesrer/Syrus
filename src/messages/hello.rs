use crate::block_exchange_protocol::Hello;
use crate::errors::{Errors, InvalidMessageErrors};
use crate::globals;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

pub fn encode_hello(msg: Hello) -> Result<BytesMut, Errors> {
    // TODO: Check whether the versioning follow semantic versioning standards

    let mut finalbuf = BytesMut::new();
    let mut encodebuf: Vec<u8> = Vec::new();

    // encode the message into encode buffer and match on the returned result
    match msg.encode(&mut encodebuf) {
        Ok(_) => {
            // Add the magic number for the Hello message to the beginning of the buffer
            finalbuf.put_i32(globals::MAGIC_NUMBER_HELLO_MESSAGE);

            // Add the size of the encoding to the beginning of the buffer
            finalbuf.put_i16(encodebuf.len() as i16);

            // read bytes from the encoding and put into the return buffer
            finalbuf.put_slice(&encodebuf);

            Ok(finalbuf)
        }
        Err(e) => Err(Errors::EncodeError(e)),
    }
}

pub fn decode_hello(mut buffer: BytesMut) -> Result<Hello, Errors> {
    // since the buffer reads in bytes, we have to divide things up into
    // byte indexes ->
    //    32 bit is 4 bytes -> Index 0 - 3
    //    16 bit is 2 bytes -> Index 0 - 1
    let magic_num = buffer.split_to(4).get_i32();
    let size = buffer.split_to(2).get_i16();

    // Throw errors if packet is unreliable
    is_hello(magic_num)?;
    verify_len(&buffer, size)?;

    match Hello::decode(buffer) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(Errors::DecodeError(e)),
    }
}

fn verify_len(buffer: &BytesMut, size: i16) -> Result<(), Errors> {
    if buffer.len() != (size as usize) {
        return Err(Errors::InvalidMessageErrors(
            InvalidMessageErrors::incorrect_length(),
        ));
    }

    Ok(())
}

fn is_hello(magic_number: i32) -> Result<(), Errors> {
    if magic_number != globals::MAGIC_NUMBER_HELLO_MESSAGE {
        return Err(Errors::InvalidMessageErrors(
            InvalidMessageErrors::invalid_hello(),
        ));
    }
    Ok(())
}
