use std::collections::HashMap;

use crate::ir::RawInstruction;

#[derive(Default, PartialEq, Debug)]
pub enum ProfileRole {
    StaticHandler,
    PureRegister,
    MutableSlot,
    ConstructedClosure,
    Rewritten,
    #[default]
    Unknown,
}

#[derive(Default)]
pub struct SlotProfile {
    pub writes: Vec<usize>,
    pub handler_refs: Vec<usize>,
    pub operand_refs: Vec<usize>,
    pub role: ProfileRole,
}

pub struct Metadata {
    profiles: HashMap<u16, SlotProfile>,
}

impl Metadata {
    pub fn query(&self, id: u16) -> Option<&SlotProfile> {
        self.profiles.get(&id)
    }
}

pub fn traverse(ingested_instructions: &[RawInstruction]) -> Metadata {
    let mut profiles: HashMap<u16, SlotProfile> = HashMap::new();

    for (idx, instruction) in ingested_instructions.iter().enumerate() {
        profiles
            .entry(instruction.destination)
            .or_insert_with(|| SlotProfile::default())
            .writes
            .push(idx);

        profiles
            .entry(instruction.opcode)
            .or_insert_with(|| SlotProfile::default())
            .handler_refs
            .push(idx);

        profiles
            .entry(instruction.operand)
            .or_insert_with(|| SlotProfile::default())
            .operand_refs
            .push(idx);
    }

    let mut static_count = 0;
    let mut register_count = 0;
    let mut mutable_count = 0;
    let mut mutable_slots: Vec<(u16, &SlotProfile)> = Vec::new();

    for (&idx, profile) in profiles.iter_mut() {
        let is_called = !profile.handler_refs.is_empty();
        let is_written = !profile.writes.is_empty();

        match (is_called, is_written) {
            (true, false) => {
                static_count += 1;
                profile.role = ProfileRole::StaticHandler;
            }
            (false, _) => {
                register_count += 1;
                profile.role = ProfileRole::PureRegister;
            }
            (true, true) => {
                mutable_count += 1;
                if idx < 128 {
                    profile.role = ProfileRole::Rewritten;
                    mutable_slots.push((idx, profile));
                } else {
                    profile.role = ProfileRole::ConstructedClosure;
                }
            }
        }
    }

    println!(
        "static handlers: {}, registers: {}, mutable: {}",
        static_count, register_count, mutable_count
    );

    mutable_slots.sort_by_key(|(idx, _)| *idx);
    for (index, profile) in &mutable_slots {
        println!(
            "  Q[{}]: {} writes, {} calls, first_write={}, first_call={}",
            index,
            profile.writes.len(),
            profile.handler_refs.len(),
            profile.writes.first().unwrap_or(&0),
            profile.handler_refs.first().unwrap_or(&0),
        );
    }

    Metadata { profiles }
}
