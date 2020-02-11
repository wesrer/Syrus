use crate::errors::{Errors, InternalError};
use crate::messages::device_id_utils::*;

#[test]
fn test_generate() {
    let alphabet = "abcdef".to_string();
    assert_eq!('e', generate(&alphabet, alphabet.clone()).unwrap());
    let alphabet = "0123456789".to_string();
    assert_eq!('3', generate(&alphabet, "7992739871".to_string()).unwrap());
}

#[test]
fn test_invalid_id() {
    let alphabet = "ABC";

    assert_eq!(
        generate(alphabet, "7992739871".to_string()),
        Err(Errors::InternalError(
            InternalError::DeviceIdGenerationError(
                "Char 7 is not valid in alphabet ABC".to_string()
            )
        ))
    );
}
