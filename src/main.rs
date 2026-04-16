mod errors;
mod handler;
mod ingest;
mod primitives;
mod resolver;
mod survey;

use errors::LiftError;

pub fn lift(encoded_bytecode: String, opcode_map_path: &str) -> Result<(), LiftError> {
    let static_map = std::fs::read_to_string(opcode_map_path).map_err(|e| LiftError::Io {
        path: opcode_map_path.to_string(),
        source: e,
    })?;

    let registry = handler::Registry::from_json(&static_map)?;
    println!("opcode map ({} entries):", registry.entries.len());
    println!("{:?}", registry.entries);

    let ingested_instructions = ingest::read(&encoded_bytecode)?;
    survey::traverse(&ingested_instructions);

    Ok(())
}

fn main() {
    let raw = std::fs::read_to_string("input/bytecode.bin").unwrap_or_else(|e| {
        eprintln!("error: failed to read input/bytecode.bin: {e}");
        std::process::exit(1);
    });
    let encoded = raw.split_whitespace().collect::<String>();

    if let Err(e) = lift(encoded, "input/static_map.json") {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
