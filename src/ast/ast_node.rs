#![allow(dead_code)]

use super::{op::*, smart_num::SmartNum};
use std::fmt::Display;

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
    pub me: AstNode<'a>,
    pub child: Vec<Expression<'a>>,
}

impl<'a> Expression<'a> {
    pub fn is_operator(&self) -> bool {
        match &self.me {
            AstNode::Operator(_) => true,
            AstNode::Operand(_) => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        match &self.me {
            AstNode::Operator(_) => false,
            AstNode::Operand(_) => true,
        }
    }

    pub fn is_num(&self) -> bool {
        match &self.me {
            AstNode::Operator(_) => false,
            AstNode::Operand(operand) => operand.is_num(),
        }
    }

    pub fn to_smart_num(&self) -> SmartNum {
        match &self.me {
            AstNode::Operator(_) => panic!("This is not a number node!"),
            AstNode::Operand(operand) => operand.to_smart_num(),
        }
    }

    fn to_string_raw(&self, upper_priority: u32) -> String {
        match &self.me {
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

    pub fn to_string(&self) -> String {
        return self.to_string_raw(0_u32);
    }

    pub fn new_variable(name: &str) -> Expression {
        Expression {
            me: AstNode::Operand(AstOperand::new_variable(name)),
            child: vec![],
        }
    }
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
