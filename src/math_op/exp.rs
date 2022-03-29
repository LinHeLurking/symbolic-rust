#![allow(dead_code)]

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_exp() -> AstOperator {
    AstOperator {
        symbol: "exp".to_string(),
        priority: 4_u32,
        descriptor: OperatorType::Exp,
    }
}

pub trait Exp {
    type Output;
    fn exp(self) -> Self::Output;
}

impl Exp for Expression {
    type Output = Expression;

    fn exp(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_exp()),
            child: vec![self],
        }
    }
}

pub fn exp(expr: Expression) -> Expression {
    expr.exp()
}
