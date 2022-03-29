#![allow(dead_code)]

use super::op::{
    operand::{AstOperand, Variable},
    operator::AstOperator,
};
use crate::smart_num::{
    rational::{RationalNum, ToRational},
    SmartNum, ToSmartNum,
};
use std::{error::Error, fmt::Display, vec};

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
                let me = if self.child.len() == 2 {
                    // binary operator
                    format!(
                        "{} {} {}",
                        self.child[0].to_string_raw(op.priority),
                        op,
                        self.child[1].to_string_raw(op.priority),
                    )
                } else {
                    // unary operator
                    format!("{}{}", op, self.child[0].to_string_raw(op.priority),)
                };
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

    pub fn is_one(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_one(),
        }
    }

    pub fn one() -> Expression {
        Expression::from(1_i64)
    }

    pub fn is_zero(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_zero(),
        }
    }

    pub fn zero() -> Expression {
        Expression::from(0_i64)
    }

    pub fn is_pi(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_pi(),
        }
    }

    pub fn pi() -> Expression {
        Expression::from(SmartNum::pi())
    }

    pub fn is_e(&self) -> bool {
        match &self.root {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_e(),
        }
    }

    pub fn e() -> Expression {
        Expression::from(SmartNum::e())
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

impl ToRational for Expression {
    type Output = Option<RationalNum>;

    fn to_rational(self) -> Self::Output {
        match self.root {
            AstNode::Operator(_) => None,
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(v) => v.to_rational(),
                AstOperand::Variable(_) => None,
            },
        }
    }
}

impl ToRational for &Expression {
    type Output = Option<RationalNum>;

    fn to_rational(self) -> Self::Output {
        match &self.root {
            AstNode::Operator(_) => None,
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(v) => v.to_rational(),
                AstOperand::Variable(_) => None,
            },
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

impl<'a> From<&'a Expression> for Option<&'a Variable> {
    fn from(expr: &'a Expression) -> Self {
        match &expr.root {
            AstNode::Operator(_) => None,
            AstNode::Operand(operand) => operand.into(),
        }
    }
}

impl From<Expression> for Option<Variable> {
    fn from(expr: Expression) -> Self {
        match expr.root {
            AstNode::Operator(_) => None,
            AstNode::Operand(operand) => operand.into(),
        }
    }
}

impl From<Variable> for Expression {
    fn from(v: Variable) -> Self {
        Expression::new_variable(v.name.as_str())
    }
}

impl From<AstOperand> for Expression {
    fn from(operand: AstOperand) -> Self {
        Expression {
            root: AstNode::Operand(operand),
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
