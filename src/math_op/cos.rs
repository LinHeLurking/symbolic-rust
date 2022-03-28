#![allow(dead_code)]

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

fn gen_cos_op() -> AstOperator {
    AstOperator {
        symbol: "cos".to_string(),
        priority: 4,
        descriptor: "Cos".to_string(),
    }
}

pub trait Cos {
    type Output;
    fn cos(self) -> Self::Output;
}

impl Cos for Expression {
    type Output = Expression;

    fn cos(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_cos_op()),
            child: vec![self],
        }
    }
}

impl Cos for &Expression {
    type Output = Expression;

    fn cos(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_cos_op()),
            child: vec![self.clone()],
        }
    }
}

pub fn cos<T: Into<Expression>>(x: T) -> Expression {
    let expr: Expression = x.into();
    expr.cos()
}

#[cfg(test)]
mod sin_tests {
    use crate::ast::ast_node::Expression;

    use super::cos;

    #[test]
    fn string_fmt() {
        let x = Expression::new_variable("x");
        let a = cos(x.clone());
        let b = cos(1);
        let c = cos(x.clone() + x.clone());
        assert_eq!(a.to_string(), "cosx");
        assert_eq!(b.to_string(), "cos1");
        assert_eq!(c.to_string(), "cos(x + x)");
    }
}
