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

impl From<i64> for AstOperand {
    fn from(k: i64) -> Self {
        AstOperand::Num(SmartNum::from(k))
    }
}

impl From<f64> for AstOperand {
    fn from(f: f64) -> Self {
        AstOperand::Num(SmartNum::from(f))
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
