#![allow(dead_code)]

use super::smart_num::SmartNum;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct AstOperator {
    pub symbol: String,
    pub priority: u32,
    pub(crate) descriptor: String,
}

impl PartialEq for AstOperator {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl Eq for AstOperator {}

impl Display for AstOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new_variable(name: &str) -> Variable {
        Variable {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstOperand {
    Num(SmartNum),
    Variable(Variable),
}

#[derive(Debug)]
pub struct OperandIsNotNumberError {}

impl Display for OperandIsNotNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operand is not a number!")
    }
}

impl Error for OperandIsNotNumberError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub static OPERAND_IS_NOT_NUMBER_ERROR: OperandIsNotNumberError = OperandIsNotNumberError {};

impl AstOperand {
    pub fn to_string(&self) -> String {
        match self {
            AstOperand::Num(num) => num.to_string(),
            AstOperand::Variable(var) => var.name.to_string(),
        }
    }

    pub fn new_variable(name: &str) -> AstOperand {
        AstOperand::Variable(Variable {
            name: String::from(name),
        })
    }

    pub fn is_num(&self) -> bool {
        match self {
            AstOperand::Num(_) => true,
            AstOperand::Variable(_) => false,
        }
    }

    pub fn to_smart_num(&self) -> Result<SmartNum, &'static OperandIsNotNumberError> {
        match &self {
            AstOperand::Num(num) => Ok(num.clone()),
            _ => Err(&OPERAND_IS_NOT_NUMBER_ERROR),
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

#[cfg(test)]
mod tests {
    use super::AstOperand;
    #[test]
    fn operand_cast() {
        let x = AstOperand::from(0_u32);
        assert!(x.to_smart_num().is_ok())
    }
}
