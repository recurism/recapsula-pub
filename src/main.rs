mod handler;
mod ingest;
mod primitives;
mod survey;

pub fn lift(encoded_bytecode: String, opcode_map_path: &str) {
    let static_map = std::fs::read_to_string(opcode_map_path)
        .unwrap_or_else(|_| panic!("failed to read {}", opcode_map_path));

    let registry = handler::Registry::from_json(&static_map).expect("failed to parse opcode map");
    println!("opcode map ({} entries):", registry.entries.len());
    println!("{:?}", registry.entries);

    let ingested_instructions = ingest::read(&encoded_bytecode);
    survey::traverse(&ingested_instructions);
}

fn main() {
    let raw =
        std::fs::read_to_string("input/bytecode.bin").expect("failed to read input/bytecode.bin");
    let encoded = raw.split_whitespace().collect::<String>();
    lift(encoded, "input/static_map.json");
}
