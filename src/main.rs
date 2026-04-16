mod ingest;
mod primitives;
mod survey;

pub fn lift(encoded_bytecode: String) {
    let ingested_instructions = ingest::read(&encoded_bytecode);
    survey::traverse(&ingested_instructions);
}

fn main() {
    let raw =
        std::fs::read_to_string("input/bytecode.bin").expect("failed to read input/bytecode.bin");
    let encoded = raw.split_whitespace().collect::<String>();
    lift(encoded);
}
