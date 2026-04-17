use crate::handler::types::SemanticSignature;

pub type Byte = u16;

pub struct RawInstruction {
    pub destination: Byte,
    pub opcode: Byte,
    pub operand: Byte,
}

pub struct Instruction {
    pub destination: u16,
    pub signature: SemanticSignature,
    pub id: u16,
}
