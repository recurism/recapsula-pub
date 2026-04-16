use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("unknown opcode {0:#04x} at instruction {1}")]
    UnknownOpcode(u16, usize),
}

#[derive(Debug, Error)]
pub enum LiftError {
    #[error("failed to read file '{path}': {source}")]
    Io { path: String, source: std::io::Error },
    #[error("invalid base64 bytecode: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("failed to parse opcode map: {0}")]
    OpcodeMap(#[from] serde_json::Error),
    #[error("resolve error: {0}")]
    Resolve(#[from] ResolveError),
}
