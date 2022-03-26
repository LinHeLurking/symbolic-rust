use std::ops::Sub;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

pub(crate) const OP_SUB: AstOperator = AstOperator {
    symbol: "-",
    priority: 2_u32,
    descriptor: "Sub",
};

impl<'a> Sub for Expression<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(OP_SUB.clone()),
            child: vec![self, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast_node::Expression, compute::evaluate::NumericEvaluate};

    #[test]
    fn sub() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x - y;
            assert_eq!(z.to_string(), "x - y");
        }
        {
            let x = Expression::from(1_i32);
            let y = Expression::from(1_i32);
            let z = (x - y).eval();
            let expected = Expression::from(0_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
