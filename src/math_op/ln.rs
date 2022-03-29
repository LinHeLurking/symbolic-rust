#![allow(dead_code)]

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_ln() -> AstOperator {
    AstOperator {
        symbol: "ln".to_string(),
        priority: 4_u32,
        descriptor: OperatorType::Ln,
    }
}

pub trait Ln {
    type Output;
    fn ln(self) -> Self::Output;
}

impl Ln for Expression {
    type Output = Expression;

    fn ln(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_ln()),
            child: vec![self],
        }
    }
}

pub fn ln(expr: Expression) -> Expression {
    expr.ln()
}
