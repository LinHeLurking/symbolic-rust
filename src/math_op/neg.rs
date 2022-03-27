use std::ops::Neg;

use crate::{
    ast::{
        ast_node::{AstNode, Expression},
        op::AstOperator,
    },
    compute::evaluate::NumericEvaluate, smart_num::ToSmartNum,
};

fn gen_op_nge() -> AstOperator {
    AstOperator {
        symbol: "-".to_string(),
        priority: 1_u32,
        descriptor: "Neg".to_string(),
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_nge()),
            child: vec![self],
        }
    }
}

pub(crate) fn neg_eval_rule(mut child: Vec<Expression>) -> Expression {
    let sub = child.pop().unwrap().eval();
    if sub.is_num() {
        Expression::from(-sub.to_smart_num().unwrap())
    } else {
        -sub
    }
}

#[cfg(test)]
mod test {
    use super::Expression;
    use crate::{compute::evaluate::NumericEvaluate, smart_num::ToSmartNum};

    #[test]
    fn neg() {
        let x = Expression::from(1_i32);
        let y = Expression::from(-1_i32);
        let z = (-y).eval();
        assert_eq!(
            x.to_smart_num().unwrap().to_i64(),
            z.to_smart_num().unwrap().to_i64()
        );
    }
}
