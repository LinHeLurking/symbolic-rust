#![allow(dead_code)]

use crate::ast::{op::AstOperator, ast_node::{Expression, AstNode}};

fn gen_sin_op() -> AstOperator {
    AstOperator {
        symbol: "sin".to_string(),
        priority: 4,
        descriptor: "Sin".to_string(),
    }
}

pub trait Sin {
    type Output;
    fn sin(self) -> Self::Output;
}

impl Sin for Expression{
    type Output = Expression;

    fn sin(self) -> Self::Output {
        Expression{
            root: AstNode::Operator(gen_sin_op()),
            child: vec![self],
        }
    }
}
