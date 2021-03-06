use crate::core::messages::generate;
use crate::errors::{Errors, InternalError};

#[test]
fn test_generating_ids() {
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
