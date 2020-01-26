use crate::block_exchange_protocol::Hello;
use crate::messages::*;

#[test]
fn test_succesful_hello_encode_decode() {
    let msg = Hello {
        device_name: "This Android".to_string(),
        client_name: "syncthing".to_string(),
        client_version: "v0.7.2".to_string(),
    };

    let buffer = encode_hello(msg.clone()).unwrap();

    assert_eq!(msg, decode_hello(buffer).unwrap());
}
