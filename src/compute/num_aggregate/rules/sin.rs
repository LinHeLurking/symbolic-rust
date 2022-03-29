use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::math_op::sin::Sin;

pub(crate) fn sin_eval_rule(mut child: Vec<Expression>) -> Expression {
    let sub = child.pop().unwrap().num_aggregate();
    match &sub.root {
        AstNode::Operand(operand) => {
            if operand.is_zero() || operand.is_pi() {
                Expression::from(0_i64)
            } else {
                sub.sin()
            }
        }
        AstNode::Operator(operator) => match operator.descriptor {
            _ => sub.sin(),
        },
    }
}
