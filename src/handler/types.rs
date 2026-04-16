use std::collections::HashMap;

use serde::Deserialize;

pub type OpcodeMap = HashMap<u16, Opcode>;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OperandSource {
    Arg { Arg: usize },
    Constant { Constant: f64 },
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
