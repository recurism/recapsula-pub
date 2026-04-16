use base64::{engine::general_purpose::STANDARD, Engine};
use base64::DecodeError;

pub fn decode(encoded_bytecode: &str) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode(encoded_bytecode)
}
