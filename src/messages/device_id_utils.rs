use crate::errors::{Errors, InternalError};
use crate::globals;

// NOTE: This module uses a slightly modified version of the Luhn's mod n
// algorithm - Instead of looping right to left, it loops left to right.
// This change was done to make the implementation consistent with the
// original Go implementation of Syncthing, where the original author made
// a mistake, but couldn't fix the bug because it would break all previous
// device ids.
pub fn generate(alphabet: &str, s: String) -> Result<char, Errors> {
    let mut factor = 1;
    let mut sum = 0;
    let n = alphabet.len();

    for ch in s.chars() {
        match alphabet.find(ch) {
            None => {
                return Err(InternalError::device_id_generation_error(ch, alphabet));
            }
            Some(codepoint) => {
                let mut addend = factor * codepoint;

                // alternate the factors
                factor = if factor == 2 { 1 } else { 2 };

                addend = (addend / n) + (addend % n);
                sum += addend;
            }
        }
    }

    let remainder = sum % n;
    let check_code_point = (n - remainder) % n;

    alphabet
        .chars()
        .nth(check_code_point)
        .ok_or(InternalError::character_fetch_error(
            check_code_point,
            alphabet,
        ))
}
