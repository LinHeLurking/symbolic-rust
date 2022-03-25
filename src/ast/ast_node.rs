#![allow(dead_code)]

use super::op::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AstNodeTag {
    Operator(AstOperator),
    Operand(AstOperand),
}

#[derive(Debug, Clone)]
pub struct AstNode {
    me: AstNodeTag,
    child: Vec<AstNode>,
}

impl AstNode {
    pub fn is_operator(&self) -> bool {
        match self.me {
            AstNodeTag::Operator(_) => true,
            AstNodeTag::Operand(_) => false,
        }
    }

    pub fn is_operand(&self) -> bool {
        match self.me {
            AstNodeTag::Operator(_) => false,
            AstNodeTag::Operand(_) => true,
        }
    }

    // Factory methods
    fn build(child: Vec<AstNode>, op_type: AstOperator) -> AstNode {
        return AstNode {
            me: AstNodeTag::Operator(op_type),
            child,
        };
    }

    pub fn neg(self) -> AstNode {
        return Self::build(vec![self], OP_NEG);
    }

    pub fn add(self, rhs: AstNode) -> AstNode {
        return Self::build(vec![self, rhs], OP_ADD);
    }

    pub fn sub(self, rhs: AstNode) -> AstNode {
        return Self::build(vec![self, rhs], OP_SUB);
    }

    pub fn mul(self, rhs: AstNode) -> AstNode {
        return Self::build(vec![self, rhs], OP_MUL);
    }

    pub fn div(self, rhs: AstNode) -> AstNode {
        return Self::build(vec![self, rhs], OP_DIV);
    }

    pub fn new_variable(name: &str) -> AstNode {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::new_variable(name)),
            child: vec![],
        }
    }

    fn to_string_raw(&self, upper_priority: u32) -> String {
        match &self.me {
            AstNodeTag::Operand(v) => v.to_string(),
            AstNodeTag::Operator(op) => {
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
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Automatically generated from python script.
impl From<f32> for AstNode {
    fn from(v: f32) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<f64> for AstNode {
    fn from(v: f64) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u8> for AstNode {
    fn from(v: u8) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i8> for AstNode {
    fn from(v: i8) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u16> for AstNode {
    fn from(v: u16) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i16> for AstNode {
    fn from(v: i16) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u32> for AstNode {
    fn from(v: u32) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i32> for AstNode {
    fn from(v: i32) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<u64> for AstNode {
    fn from(v: u64) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

impl From<i64> for AstNode {
    fn from(v: i64) -> Self {
        AstNode {
            me: AstNodeTag::Operand(AstOperand::from(v)),
            child: vec![],
        }
    }
}

#[test]
fn ast_fmt() {
    {
        let a = AstNode::from(1_i64);
        let b = AstNode::from(2_f64);
        let c = a.add(b);
        let expected = "1 + 2.000";
        assert_eq!(c.to_string(), expected);
        assert_eq!(format!("{}", c), expected);
    }
    {
        let a = AstNode::from(1_f64);
        let b = AstNode::from(2_i64);
        let c = a.add(b);
        let expected = "1.000 + 2";
        assert_eq!(c.to_string(), expected);
        assert_eq!(format!("{}", c), expected);
    }
    {
        let a = AstNode::from(1_f64);
        let b = AstNode::from(2_i64);
        let c = AstNode::from(3_i64);
        let d = c.mul(a.add(b));
        let expected = "3 * (1.000 + 2)";
        assert_eq!(d.to_string(), expected);
        assert_eq!(format!("{}", d), expected);
    }
    {
        let a = AstNode::new_variable("x");
        let expected = "x";
        assert_eq!(a.to_string(), expected);
        assert_eq!(format!("{}", a), expected);
    }
}
