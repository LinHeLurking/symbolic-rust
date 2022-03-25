use super::smart_num::SmartNum;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AstOperatorType {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct AstOperator {
    pub priority: u32,
    pub op_type: AstOperatorType,
}

impl AstOperator {
    pub fn to_string(&self) -> String {
        match &self.op_type {
            AstOperatorType::Neg => String::from("-"),
            AstOperatorType::Add => String::from("+"),
            AstOperatorType::Sub => String::from("-"),
            AstOperatorType::Mul => String::from("*"),
            AstOperatorType::Div => String::from("/"),
        }
    }
}

impl Display for AstOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub const OP_NEG: AstOperator = AstOperator {
    priority: 1,
    op_type: AstOperatorType::Neg,
};

pub const OP_ADD: AstOperator = AstOperator {
    priority: 2,
    op_type: AstOperatorType::Add,
};

pub const OP_SUB: AstOperator = AstOperator {
    priority: 2,
    op_type: AstOperatorType::Sub,
};

pub const OP_MUL: AstOperator = AstOperator {
    priority: 3,
    op_type: AstOperatorType::Mul,
};

pub const OP_DIV: AstOperator = AstOperator {
    priority: 3,
    op_type: AstOperatorType::Div,
};

// TODO: Add variables as operands.
#[derive(Debug, Clone)]
pub enum AstOperand {
    Num(SmartNum),
}

impl AstOperand {
    pub fn to_string(&self) -> String {
        match self {
            AstOperand::Num(num) => num.to_string(),
        }
    }
}

// Automatically generated from python script.
impl From<f32> for AstOperand{
    fn from(v: f32) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<f64> for AstOperand{
    fn from(v: f64) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<u8> for AstOperand{
    fn from(v: u8) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<i8> for AstOperand{
    fn from(v: i8) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<u16> for AstOperand{
    fn from(v: u16) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<i16> for AstOperand{
    fn from(v: i16) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<u32> for AstOperand{
    fn from(v: u32) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<i32> for AstOperand{
    fn from(v: i32) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<u64> for AstOperand{
    fn from(v: u64) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

impl From<i64> for AstOperand{
    fn from(v: i64) -> Self {
        AstOperand::Num(SmartNum::from(v))
    }
}

#[test]
fn op_to_fmt() {
    let check_table = vec![
        (OP_NEG, "-"),
        (OP_ADD, "+"),
        (OP_SUB, "-"),
        (OP_MUL, "*"),
        (OP_DIV, "/"),
    ];
    for (op, expected) in check_table {
        assert_eq!(op.to_string(), expected);
        assert_eq!(format!("{}", op), expected);
    }
}
