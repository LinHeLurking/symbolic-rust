#![allow(dead_code)]

use super::{op::*, smart_num::SmartNum};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

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
    pub me: AstNode,
    pub child: Vec<Expression>,
}

impl Expression {
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

    // Factory methods
    fn build(child: Vec<Expression>, op_type: AstOperator) -> Expression {
        return Expression {
            me: AstNode::Operator(op_type),
            child,
        };
    }

    pub fn neg(self) -> Expression {
        return Self::build(vec![self], AstOperator::gen_neg_op());
    }

    pub fn add(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], AstOperator::gen_add_op());
    }

    pub fn sub(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], AstOperator::gen_sub_op());
    }

    pub fn mul(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], AstOperator::gen_mul_op());
    }

    pub fn div(self, rhs: Expression) -> Expression {
        return Self::build(vec![self, rhs], AstOperator::gen_div_op());
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

impl Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}

impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl<T> From<T> for Expression
where
    T: Into<SmartNum>,
{
    fn from(v: T) -> Self {
        Expression {
            me: AstNode::from(v),
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
