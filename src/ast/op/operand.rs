#![allow(dead_code)]

use crate::smart_num::*;
use std::{error::Error, fmt::Display};

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

pub(crate) trait ToVariable {
    fn to_variable(self) -> Option<Variable>;
}

impl ToVariable for &Variable {
    fn to_variable(self) -> Option<Variable> {
        Some(self.clone())
    }
}

impl ToVariable for Variable {
    fn to_variable(self) -> Option<Variable> {
        Some(self)
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

    pub fn is_one(&self) -> bool {
        match self {
            AstOperand::Variable(_) => false,
            AstOperand::Num(num) => num.is_one(),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            AstOperand::Variable(_) => false,
            AstOperand::Num(num) => num.is_zero(),
        }
    }

    pub fn is_pi(&self) -> bool {
        match self {
            AstOperand::Variable(_) => false,
            AstOperand::Num(num) => num.is_pi(),
        }
    }

    pub fn is_e(&self) -> bool {
        match self {
            AstOperand::Variable(_) => false,
            AstOperand::Num(num) => num.is_e(),
        }
    }
}

impl Display for AstOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToSmartNum for AstOperand {
    type Output = Result<SmartNum, &'static OperandIsNotNumberError>;
    fn to_smart_num(self) -> Self::Output {
        match self {
            AstOperand::Num(num) => Ok(num),
            _ => Err(&OPERAND_IS_NOT_NUMBER_ERROR),
        }
    }
}

impl<'a> ToSmartNum for &'a AstOperand {
    type Output = Result<&'a SmartNum, &'static OperandIsNotNumberError>;
    fn to_smart_num(self) -> Self::Output {
        match self {
            AstOperand::Num(num) => Ok(num),
            _ => Err(&OPERAND_IS_NOT_NUMBER_ERROR),
        }
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
    use crate::smart_num::ToSmartNum;

    use super::AstOperand;
    #[test]
    fn operand_cast() {
        let x = AstOperand::from(0_u32);
        assert!(x.to_smart_num().is_ok())
    }
}
