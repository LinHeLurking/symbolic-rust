use std::ops::Neg;

use crate::ast::{
    op::operator::{AstOperator, OperatorType},
    tree::{AstNode, Expression},
};

fn gen_op_neg() -> AstOperator {
    AstOperator {
        symbol: "-".to_string(),
        priority: 1_u32,
        descriptor: OperatorType::Neg,
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_neg()),
            child: vec![self],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Expression;
    use crate::{compute::num_aggregate::NumAggregate, smart_num::ToSmartNum};

    #[test]
    fn neg() {
        let x = Expression::from(1_i32);
        let y = Expression::from(-1_i32);
        let z = (-y).num_aggregate();
        assert_eq!(
            x.to_smart_num().unwrap().to_i64(),
            z.to_smart_num().unwrap().to_i64()
        );
    }
}
