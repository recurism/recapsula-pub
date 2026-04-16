use crate::primitives::Byte;

mod decoder;

pub struct RawInstruction {
    pub destination: Byte,
    pub opcode: Byte,
    pub operand: Byte,
}

pub fn read(encoded_bytecode: &str) -> Vec<RawInstruction> {
    let decoded_bytecode = decoder::decode(encoded_bytecode);

    let u16s: Vec<u16> = decoded_bytecode
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();

    let ret: Vec<RawInstruction> = u16s
        .chunks(3)
        .map(|chunk| RawInstruction {
            destination: chunk[0],
            opcode: chunk[1],
            operand: chunk[2],
        })
        .collect();

    println!("[Lifter] Ingested {:?} instructions", ret.len());
    ret
}
