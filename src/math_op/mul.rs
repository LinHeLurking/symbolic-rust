use std::ops::Mul;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

pub(crate) const OP_MUL: AstOperator = AstOperator {
    symbol: "*",
    priority: 3_u32,
    descriptor: "Mul",
};

impl<'a> Mul for Expression<'a> {
    type Output = Expression<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(OP_MUL.clone()),
            child: vec![self, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast_node::Expression, compute::evaluate::NumericEvaluate};

    #[test]
    fn mul() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x * y;
            assert_eq!(z.to_string(), "x * y");
        }
        {
            let x = Expression::from(2_i32);
            let y = Expression::from(2_i32);
            let z = (x * y).eval();
            let expected = Expression::from(4_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
