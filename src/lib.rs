#[cfg(test)]
mod tests;

// Include the `items` module, which is generated from items.proto.
pub mod block_exchange_protocol {
    include!(concat!(env!("OUT_DIR"), "/BEP.protobufs.rs"));
}
