use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("no profile for slot {0:#04x} at instruction {1}")]
    UnknownOpcode(u16, usize),
    #[error("byte {0} is a static handler but has no registry entry (instruction {1})")]
    UnregisteredHandler(u16, usize),
}

#[derive(Debug, Error)]
pub enum LiftError {
    #[error("failed to read file '{path}': {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid base64 bytecode: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("failed to parse opcode map: {0}")]
    OpcodeMap(#[from] serde_json::Error),
    #[error("resolve error: {0}")]
    Resolve(#[from] ResolveError),
}
