#![allow(dead_code)]

use super::{op::*, smart_num::SmartNum};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum AstNode<'a> {
    Operator(AstOperator<'a>),
    Operand(AstOperand<'a>),
}

impl<'a, T> From<T> for AstNode<'a>
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        AstNode::Operand(AstOperand::from(v))
    }
}

#[derive(Debug, Clone)]
pub struct Expression<'a> {
    pub root: AstNode<'a>,
    pub child: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct ExprIsNotNumError<'a> {
    pub expr: &'a Expression<'a>,
    pub source_: Option<&'static dyn Error>,
}

impl<'a> Display for ExprIsNotNumError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a number!", self.expr)
    }
}

impl<'a> Error for ExprIsNotNumError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source_
    }
}

impl<'a> Expression<'a> {
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

    pub fn to_smart_num(&self) -> Result<SmartNum, ExprIsNotNumError> {
        match &self.root {
            AstNode::Operator(_) => Err(ExprIsNotNumError {
                expr: self,
                source_: None,
            }),
            AstNode::Operand(operand) => operand.to_smart_num().map_err(|e| ExprIsNotNumError {
                expr: self,
                source_: Some(e),
            }),
        }
    }

    pub fn near(&'a self, another: &'a Expression, eps: f64) -> Result<bool, ExprIsNotNumError> {
        let x = self.to_smart_num()?;
        let y = another.to_smart_num()?;
        return Ok((x-y).to_f64().abs() < eps);
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

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_raw(0_u32))
    }
}

impl<'a, T> From<T> for Expression<'a>
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
    use super::Expression;
    #[test]
    fn num_cast() {
        let x = Expression::from(0_u32);
        let y = Expression::new_variable("x");
        assert!(x.to_smart_num().is_ok());
        assert!(y.to_smart_num().is_err());
    }
}
