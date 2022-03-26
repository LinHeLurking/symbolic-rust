#![allow(dead_code)]

use super::smart_num::SmartNum;
use std::{fmt::Display, panic};

#[derive(Debug, Clone, Copy)]
pub struct AstOperatorBase {
    name: &'static str,
    priority: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AstOperator {
    Neg(AstOperatorBase),
    Add(AstOperatorBase),
    Sub(AstOperatorBase),
    Mul(AstOperatorBase),
    Div(AstOperatorBase),
}

impl AstOperator {
    fn to_base(&self) -> &AstOperatorBase {
        match self {
            Self::Neg(op) | Self::Add(op) | Self::Sub(op) | Self::Mul(op) | Self::Div(op) => op,
        }
    }

    pub fn to_string(&self) -> &str {
        self.to_base().name
    }

    pub fn priority(&self) -> u32 {
        self.to_base().priority.clone()
    }

    pub fn gen_neg_op() -> AstOperator {
        AstOperator::Neg(AstOperatorBase {
            name: "-",
            priority: 1,
        })
    }

    pub fn gen_add_op() -> AstOperator {
        AstOperator::Add(AstOperatorBase {
            name: "+",
            priority: 2,
        })
    }

    pub fn gen_sub_op() -> AstOperator {
        AstOperator::Sub(AstOperatorBase {
            name: "-",
            priority: 2,
        })
    }

    pub fn gen_mul_op() -> AstOperator {
        AstOperator::Mul(AstOperatorBase {
            name: "*",
            priority: 3,
        })
    }

    pub fn gen_div_op() -> AstOperator {
        AstOperator::Div(AstOperatorBase {
            name: "/",
            priority: 3,
        })
    }
}

impl Display for AstOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub(crate) name: String,
}

impl Variable {
    pub fn new_variable(name: &str) -> Variable {
        Variable {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstOperand {
    Num(SmartNum),
    Variable(Variable),
}

impl AstOperand {
    pub fn to_string(&self) -> String {
        match self {
            AstOperand::Num(num) => num.to_string(),
            AstOperand::Variable(var) => var.name.clone(),
        }
    }

    pub fn new_variable(name: &str) -> AstOperand {
        AstOperand::Variable(Variable {
            name: name.to_string(),
        })
    }

    pub fn is_num(&self) -> bool {
        match self {
            AstOperand::Num(_) => true,
            AstOperand::Variable(_) => false,
        }
    }

    pub fn to_smart_num(&self) -> SmartNum {
        match &self {
            AstOperand::Num(num) => num.clone(),
            _ => panic!("This is not a number!"),
        }
    }
}

impl Display for AstOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> From<T> for AstOperand
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        AstOperand::Num(v.into())
    }
}

#[test]
fn operator_to_fmt() {
    let check_table = vec![
        (AstOperator::gen_neg_op(), "-"),
        (AstOperator::gen_add_op(), "+"),
        (AstOperator::gen_sub_op(), "-"),
        (AstOperator::gen_mul_op(), "*"),
        (AstOperator::gen_div_op(), "/"),
    ];
    for (op, expected) in check_table {
        assert_eq!(op.to_string(), expected);
        assert_eq!(format!("{}", op), expected);
    }
}

#[test]
fn operand_to_fmt() {
    {
        let x = AstOperand::from(1_i64);
        assert_eq!(x.to_string(), "1");
        assert_eq!(format!("{}", x), "1");
    }
    {
        let x = AstOperand::from(1_f64);
        assert_eq!(x.to_string(), "1.000");
        assert_eq!(format!("{}", x), "1.000");
    }
    {
        let x = AstOperand::new_variable("x");
        assert_eq!(x.to_string(), "x");
        assert_eq!(format!("{}", x), "x");
    }
}
