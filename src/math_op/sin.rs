#![allow(dead_code)]

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_sin_op() -> AstOperator {
    AstOperator {
        symbol: "sin".to_string(),
        priority: 5,
        descriptor: OperatorType::Sin,
    }
}

pub trait Sin {
    type Output;
    fn sin(self) -> Self::Output;
}

impl Sin for Expression {
    type Output = Expression;

    fn sin(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_sin_op()),
            child: vec![self],
        }
    }
}

impl Sin for &Expression {
    type Output = Expression;

    fn sin(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_sin_op()),
            child: vec![self.clone()],
        }
    }
}

pub fn sin<T: Into<Expression>>(x: T) -> Expression {
    let expr: Expression = x.into();
    expr.sin()
}

#[cfg(test)]
mod sin_tests {
    use crate::ast::tree::Expression;

    use super::sin;

    #[test]
    fn string_fmt() {
        let x = Expression::new_variable("x");
        let a = sin(x.clone());
        let b = sin(1);
        let c = sin(x.clone() + x.clone());
        assert_eq!(a.to_string(), "sinx");
        assert_eq!(b.to_string(), "sin1");
        assert_eq!(c.to_string(), "sin(x + x)");
    }
}
