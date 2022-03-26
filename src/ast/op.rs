#![allow(dead_code)]

use super::smart_num::SmartNum;
use std::{fmt::Display, panic};

#[derive(Debug, Clone)]
pub struct AstOperator<'a> {
    pub symbol: &'a str,
    pub priority: u32,
    pub(crate) descriptor: &'a str,
}

impl<'a> PartialEq for AstOperator<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl<'a> Eq for AstOperator<'a> {}

impl<'a> AstOperator<'a> {
    pub fn to_string(&self) -> String {
        self.symbol.to_string()
    }
}

impl<'a> Display for AstOperator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    pub name: &'a str,
}

impl<'a> Variable<'a> {
    pub fn new_variable(name: &'a str) -> Variable {
        Variable { name }
    }
}

#[derive(Debug, Clone)]
pub enum AstOperand<'a> {
    Num(SmartNum),
    Variable(Variable<'a>),
}

impl<'a> AstOperand<'a> {
    pub fn to_string(&self) -> String {
        match self {
            AstOperand::Num(num) => num.to_string(),
            AstOperand::Variable(var) => var.name.to_string(),
        }
    }

    pub fn new_variable(name: &'a str) -> AstOperand {
        AstOperand::Variable(Variable { name })
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

impl<'a> Display for AstOperand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'a, T> From<T> for AstOperand<'a>
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        AstOperand::Num(v.into())
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
