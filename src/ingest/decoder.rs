use base64::{Engine, engine::general_purpose::STANDARD};

pub fn decode(encoded_bytecode: &str) -> Vec<u8> {
    STANDARD.decode(encoded_bytecode).expect("invalid base64")
}
