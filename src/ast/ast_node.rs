#![allow(dead_code)]

use super::{op::*, smart_num::SmartNum};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(AstOperator),
    Operand(AstOperand),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub me: AstNode,
    pub child: Vec<Expression>,
}

impl Expression {
    pub fn is_operator(&self) -> bool {
        match self.me {
            AstNode::Operator(_) => true,
            AstNode::Operand(_) => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        match self.me {
            AstNode::Operator(_) => false,
            AstNode::Operand(_) => true,
        }
    }

    // Factory methods
    fn build(child: Vec<Expression>, op_type: AstOperator) -> Expression {
        return Expression {
            me: AstNode::Operator(op_type),
            child,
        };
    }

    pub fn neg(self) -> Expression {
        return Self::build(vec![self], OP_NEG);
    }

    pub fn add(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], OP_ADD);
    }

    pub fn sub(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], OP_SUB);
    }

    pub fn mul(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], OP_MUL);
    }

    pub fn div(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], OP_DIV);
    }

    pub fn new_variable(name: &str) -> Expression {
        Expression {
            me: AstNode::Operand(AstOperand::new_variable(name)),
            child: vec![],
        }
    }

    fn to_string_raw(&self, upper_priority: u32) -> String {
        match &self.me {
            AstNode::Operand(v) => v.to_string(),
            AstNode::Operator(op) => {
                let me = format!(
                    "{} {} {}",
                    self.child[0].to_string_raw(op.priority()),
                    op,
                    self.child[1].to_string_raw(op.priority()),
                );
                if op.priority() < upper_priority {
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
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<SmartNum> for Expression {
    fn from(v: SmartNum) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

// Automatically generated from python script.
impl From<f32> for Expression {
    fn from(v: f32) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u8> for Expression {
    fn from(v: u8) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i8> for Expression {
    fn from(v: i8) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u16> for Expression {
    fn from(v: u16) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i16> for Expression {
    fn from(v: i16) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u32> for Expression {
    fn from(v: u32) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i32> for Expression {
    fn from(v: i32) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u64> for Expression {
    fn from(v: u64) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i64> for Expression {
    fn from(v: i64) -> Self {
        Expression {
            me: AstNode::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

#[test]
fn ast_fmt() {
    {
        let a = Expression::from(1_i64);
        let b = Expression::from(2_f64);
        let c = a.add(b);
        let expected = "1 + 2.000";
        assert_eq!(c.to_string(), expected);
        assert_eq!(format!("{}", c), expected);
    }
    {
        let a = Expression::from(1_f64);
        let b = Expression::from(2_i64);
        let c = a.add(b);
        let expected = "1.000 + 2";
        assert_eq!(c.to_string(), expected);
        assert_eq!(format!("{}", c), expected);
    }
    {
        let a = Expression::from(1_f64);
        let b = Expression::from(2_i64);
        let c = Expression::from(3_i64);
        let d = c.mul(a.add(b));
        let expected = "3 * (1.000 + 2)";
        assert_eq!(d.to_string(), expected);
        assert_eq!(format!("{}", d), expected);
    }
    {
        let a = Expression::new_variable("x");
        let expected = "x";
        assert_eq!(a.to_string(), expected);
        assert_eq!(format!("{}", a), expected);
    }
}
