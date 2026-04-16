use crate::{
    errors::ResolveError, handler::types::SemanticSignature, ingest::RawInstruction,
    survey::Metadata,
};

pub struct Instruction {
    destination: u16,
    signature: SemanticSignature,
    id: u16,
}

pub struct Resolver<'a> {
    raw_instructions: &'a [RawInstruction],
    metadata: &'a Metadata,
    pub instructions: Vec<Instruction>,
}

impl<'a> Resolver<'a> {
    pub fn new(raw_instructions: &'a [RawInstruction], metadata: &'a Metadata) -> Self {
        Self {
            metadata,
            raw_instructions,
            instructions: vec![],
        }
    }

    pub fn handle_instructions(&mut self) -> Result<(), ResolveError> {
        for (idx, instruction) in self.raw_instructions.iter().enumerate() {
            self.handle_instruction(idx, instruction)?;
        }
        Ok(())
    }

    fn handle_instruction(
        &mut self,
        idx: usize,
        instruction: &RawInstruction,
    ) -> Result<(), ResolveError> {
        let opcode_profile = self
            .metadata
            .query(instruction.opcode)
            .ok_or(ResolveError::UnknownOpcode(instruction.opcode, idx))?;

        match opcode_profile.role {
            _ => {}
        }

        Ok(())
    }
}
