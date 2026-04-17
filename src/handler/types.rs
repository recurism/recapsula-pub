use std::collections::HashMap;

use serde::Deserialize;

pub type OpcodeMap = HashMap<u16, Opcode>;

#[derive(Debug, Clone)]
pub enum ResolvedOperand {
    Arg(usize),
    Constant(f64),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OperandSource {
    Arg { #[serde(rename = "Arg")] arg: usize },
    Constant { #[serde(rename = "Constant")] constant: f64 },
}

impl OperandSource {
    pub fn decode(&self) -> ResolvedOperand {
        match self {
            OperandSource::Arg { arg: n } => ResolvedOperand::Arg(*n),
            OperandSource::Constant { constant: v } => ResolvedOperand::Constant(*v),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    ShiftLeft,
    ShiftRight,
    UnsignedShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    StrictEq,
    StrictNeq,
    Lt,
    Gt,
    Lte,
    Gte,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum SemanticSignature {
    Binary {
        op: BinaryOperator,
        lhs: OperandSource,
        rhs: OperandSource,
    },
    Call {
        callee: OperandSource,
        arguments: Vec<OperandSource>,
    },
    Member {
        key: OperandSource,
    },
    Load {
        src: OperandSource,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Opcode {
    pub id: u16,
    pub arity: usize,
    pub depth: usize,
    pub signature: SemanticSignature,
}
