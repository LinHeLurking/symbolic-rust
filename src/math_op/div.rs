use std::ops::Div;

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_div() -> AstOperator {
    AstOperator {
        symbol: "/".to_string(),
        priority: 3_u32,
        descriptor: OperatorType::Div,
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
    use crate::{ast::tree::Expression, compute::num_aggregate::NumAggregate};

    #[test]
    fn div() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x / y;
            assert_eq!(z.to_string(), "x / y");
        }
        {
            let x = Expression::from(2_i32);
            let y = Expression::from(2_i32);
            let z = (x / y).num_aggregate();
            let expected = Expression::from(1_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
