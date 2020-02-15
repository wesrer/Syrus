pub const MAGIC_NUMBER_HELLO_MESSAGE: i32 = 0x2EA7D90B;

// this is a string slice type
// This is the alphabet for the Luhn mod n algorihtm that our implementation is using.
// For more details, see: https://en.wikipedia.org/wiki/Luhn_mod_N_algorithm
pub const LUHN_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

// shifts
pub const KIB: u32 = 10;
pub const MIB: u32 = 20;
pub const GIB: u32 = 30;

// Maximum message length allowed on the wire (500 MB)
pub const MAX_MESSAGE_LEN: u32 = 500 * 1000 * 1000;

// Minimum Block size allowed
pub const MIN_BLOCK_SIZE: u32 = 128 << KIB;

// Maximum Block size allowed
pub const MAX_BLOCK_SIZE: u32 = 16 << MIB;

// Number of blocks we aim per file
pub const DESIRED_BLOCKS: u32 = 2000;

// TODO: Copy the sha256 of a block with all zeros
