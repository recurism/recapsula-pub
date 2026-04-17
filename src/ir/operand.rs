#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(u16),
    Handler(u16),
    Constant(ConstantValue),
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
    Undefined,
}
