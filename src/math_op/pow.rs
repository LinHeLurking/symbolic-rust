#![allow(dead_code)]

use std::vec;

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_pow() -> AstOperator {
    AstOperator {
        symbol: "^".to_string(),
        priority: 4_u32,
        descriptor: OperatorType::Pow,
    }
}

pub trait Pow<T> {
    type Output;
    fn pow(self, r: T) -> Self::Output;
}

impl<T> Pow<T> for Expression
where
    T: Into<Expression>,
{
    type Output = Expression;

    fn pow(self, r: T) -> Self::Output {
        let a: Expression = r.into();
        Expression {
            root: AstNode::Operator(gen_op_pow()),
            child: vec![self, a],
        }
    }
}
