#![allow(dead_code)]

use crate::{
    ast::{
        ast_node::{AstNode, Expression},
        op::{AstOperator, Variable},
    },
    compute::derivative::Derivative,
};

use super::cos::cos;

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

pub(crate) fn sin_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (sin(u))' = cos(u)*u'
    let sub = child.pop().unwrap();
    cos(sub.clone()) * sub.derivative(to)
}

#[cfg(test)]
mod sin_tests {
    use crate::ast::ast_node::Expression;

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
