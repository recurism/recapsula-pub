mod ingest;
mod primitives;

pub fn lift(encoded_bytecode: String) {
    let ingested_instructions = ingest::read(&encoded_bytecode);
}

fn main() {
    let raw =
        std::fs::read_to_string("input/bytecode.bin").expect("failed to read input/bytecode.bin");
    let encoded = raw.split_whitespace().collect::<String>();
    lift(encoded);
}
