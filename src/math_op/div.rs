use std::ops::Div;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

fn gen_op_div() -> AstOperator {
    AstOperator {
        symbol: "/".to_string(),
        priority: 3_u32,
        descriptor: "Div".to_string(),
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_div()),
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
            let z = x / y;
            assert_eq!(z.to_string(), "x / y");
        }
        {
            let x = Expression::from(2_i32);
            let y = Expression::from(2_i32);
            let z = (x / y).eval();
            let expected = Expression::from(1_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}