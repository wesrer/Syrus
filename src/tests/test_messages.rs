use crate::block_exchange_protocol::{Header, Hello, MessageCompression, MessageType};
use crate::messages::{Decode, Encode};

#[test]
fn test_succesful_hello_encode_decode() {
    let msg = Hello {
        device_name: "This Android".to_string(),
        client_name: "syncthing".to_string(),
        client_version: "v0.7.2".to_string(),
    };

    let mut buffer = msg.encode_to_bytes().unwrap();

    assert_eq!(msg, Hello::decode_from(&mut buffer).unwrap());
}

#[test]
fn test_succesful_header_encode_decode() {
    let header_without_compression = Header {
        r#type: MessageType::DownloadProgress as i32,
        compression: MessageCompression::None as i32,
    };

    let header_with_compression = Header {
        r#type: MessageType::DownloadProgress as i32,
        compression: MessageCompression::None as i32,
    };
    Header {
        r#type: MessageType::IndexUpdate as i32,
        compression: MessageCompression::Lz4 as i32,
    };

    let mut buffer = header_without_compression.encode_to_bytes().unwrap();

    assert_eq!(
        header_without_compression,
        Header::decode_from(&mut buffer).unwrap()
    );

    let mut buffer = header_with_compression.encode_to_bytes().unwrap();

    assert_eq!(
        header_with_compression,
        Header::decode_from(&mut buffer).unwrap()
    );
}
