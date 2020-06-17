use crate::block_exchange_protocol::{Header, Hello, MessageCompression, MessageType};
use crate::messages::{Decode, Encode};

#[test]
fn test_hello_message_encode_decode() {
    let msg = Hello {
        device_name: "This Android".to_owned(),
        client_name: "syncthing".to_owned(),
        client_version: "v0.7.2".to_owned(),
    };

    // This is the test example used in the official syncthing Go implementation
    let syncthing_test = Hello {
        device_name: "test_device".to_owned(),
        client_name: "syncthing".to_owned(),
        client_version: "v0.14.5".to_owned(),
    };

    let mut buffer = msg.encode_to_bytes().unwrap();
    let mut buffer2 = syncthing_test.encode_to_bytes().unwrap();

    assert_eq!(msg, Hello::decode_from(&mut buffer).unwrap());
    assert_eq!(syncthing_test, Hello::decode_from(&mut buffer2).unwrap());
}

#[test]
fn test_message_header_encode_decode() {
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
