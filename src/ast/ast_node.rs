#![allow(dead_code)]

use super::op::*;
use crate::smart_num::{SmartNum, ToSmartNum};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(AstOperator),
    Operand(AstOperand),
}

impl<T> From<T> for AstNode
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        AstNode::Operand(AstOperand::from(v))
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub root: AstNode,
    pub child: Vec<Expression>,
}

#[derive(Debug)]
pub struct ExprIsNotNumError {
    pub source_: Option<&'static dyn Error>,
}

impl Display for ExprIsNotNumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This expression is not a number!")
    }
}

impl Error for ExprIsNotNumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source_
    }
}

impl Expression {
    pub fn is_operator(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => true,
            AstNode::Operand(_) => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(_) => true,
        }
    }

    pub fn is_num(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_num(),
        }
    }

    pub fn near(&self, another: &Expression, eps: f64) -> Result<bool, ExprIsNotNumError> {
        let x = self.to_smart_num()?;
        let y = another.to_smart_num()?;
        return Ok((*x - *y).to_f64().abs() < eps);
    }

    fn to_string_raw(&self, upper_priority: u32) -> String {
        match &self.root {
            AstNode::Operand(v) => v.to_string(),
            AstNode::Operator(op) => {
                let me = format!(
                    "{} {} {}",
                    self.child[0].to_string_raw(op.priority),
                    op,
                    self.child[1].to_string_raw(op.priority),
                );
                if op.priority < upper_priority {
                    format!("({})", me)
                } else {
                    me
                }
            }
        }
    }

    pub fn new_variable(name: &str) -> Expression {
        Expression {
            root: AstNode::Operand(AstOperand::new_variable(name)),
            child: vec![],
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_raw(0_u32))
    }
}

impl ToSmartNum for Expression {
    type Output = Result<SmartNum, ExprIsNotNumError>;
    fn to_smart_num(self) -> Self::Output {
        match self.root {
            AstNode::Operator(_) => Err(ExprIsNotNumError { source_: None }),
            AstNode::Operand(operand) => operand
                .to_smart_num()
                .map_err(|e| ExprIsNotNumError { source_: Some(e) }),
        }
    }
}

impl<'a> ToSmartNum for &'a Expression {
    type Output = Result<&'a SmartNum, ExprIsNotNumError>;
    fn to_smart_num(self) -> Self::Output {
        match &self.root {
            AstNode::Operator(_) => Err(ExprIsNotNumError { source_: None }),
            AstNode::Operand(operand) => operand
                .to_smart_num()
                .map_err(|e| ExprIsNotNumError { source_: Some(e) }),
        }
    }
}

impl<T> From<T> for Expression
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        Expression {
            root: AstNode::from(v),
            child: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_num::ToSmartNum;

    use super::Expression;
    #[test]
    fn num_cast() {
        let x = Expression::from(0_u32);
        let y = Expression::new_variable("x");
        assert!(x.to_smart_num().is_ok());
        assert!(y.to_smart_num().is_err());
    }
}
