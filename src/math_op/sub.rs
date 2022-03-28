use std::ops::Sub;

use crate::{
    ast::{
        ast_node::{AstNode, Expression},
        op::AstOperator,
    },
    compute::num_aggregate::NumAggregate,
    smart_num::ToSmartNum,
};

fn gen_op_sub() -> AstOperator {
    AstOperator {
        symbol: "-".to_string(),
        priority: 2_u32,
        descriptor: "Sub".to_string(),
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_sub()),
            child: vec![self, rhs],
        }
    }
}

pub(crate) fn sub_eval_rule(mut child: Vec<Expression>) -> Expression {
    let r = child.pop().unwrap().num_aggregate();
    let l = child.pop().unwrap().num_aggregate();
    if l.is_num() && r.is_num() {
        Expression::from(l.to_smart_num().unwrap() - r.to_smart_num().unwrap())
    } else if l.is_zero() {
        r
    } else if r.is_zero() {
        l
    } else {
        l - r
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast_node::Expression, compute::num_aggregate::NumAggregate};

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
            let z = (x - y).num_aggregate();
            let expected = Expression::from(0_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
