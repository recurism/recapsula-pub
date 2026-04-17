use crate::{
    errors::ResolveError,
    handler::{
        Registry,
        types::Opcode,
    },
    ir::{Instruction, RawInstruction},
    survey::{Metadata, ProfileRole},
};

pub struct Resolver<'a> {
    raw_instructions: &'a [RawInstruction],
    metadata: &'a Metadata,
    registry: &'a Registry,
    pub instructions: Vec<Instruction>,
}

impl<'a> Resolver<'a> {
    pub fn new(
        raw_instructions: &'a [RawInstruction],
        metadata: &'a Metadata,
        registry: &'a Registry,
    ) -> Self {
        Self {
            metadata,
            raw_instructions,
            registry,
            instructions: vec![],
        }
    }

    pub fn handle_instructions(&mut self) -> Result<(), ResolveError> {
        for (idx, instruction) in self.raw_instructions.iter().take(10).enumerate() {
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
            ProfileRole::StaticHandler => {
                self.parse_static_handler(instruction, idx)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_static_handler(
        &mut self,
        instruction: &RawInstruction,
        idx: usize,
    ) -> Result<(), ResolveError> {
        let opcode = self
            .registry
            .get(instruction.opcode)
            .ok_or(ResolveError::UnregisteredHandler(instruction.opcode, idx))?;

        // emit
        if opcode.depth == 1 {
            println!("{:?}", opcode)
        }

        println!("{:?}", opcode);
        Ok(())
    }
}
