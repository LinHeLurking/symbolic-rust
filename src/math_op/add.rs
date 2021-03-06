use std::ops::Add;

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_add() -> AstOperator {
    AstOperator {
        symbol: "+".to_string(),
        priority: 2_u32,
        descriptor: OperatorType::Add,
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_add()),
            child: vec![self, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::tree::Expression, compute::num_aggregate::NumAggregate};

    #[test]
    fn add() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x + y;
            assert_eq!(z.to_string(), "x + y");
        }
        {
            let x = Expression::from(1_u32);
            let y = Expression::from(1_u32);
            let z = (x + y).num_aggregate();
            let expected = Expression::from(2_u32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
