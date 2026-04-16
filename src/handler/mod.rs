pub mod types;

use std::collections::HashMap;

use crate::handler::types::{Opcode, OpcodeMap};

pub struct Registry {
    pub entries: HashMap<u16, Opcode>,
}

impl Registry {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let mut entries: OpcodeMap = OpcodeMap::new();

        let opcodes: Vec<Opcode> = serde_json::from_str(json)?;
        for entry in opcodes {
            entries.insert(entry.id, entry);
        }

        Ok(Self { entries })
    }

    pub fn get(&self, id: u16) -> Option<&Opcode> {
        self.entries.get(&id)
    }

    pub fn contains(&self, id: u16) -> bool {
        self.entries.contains_key(&id)
    }
}
