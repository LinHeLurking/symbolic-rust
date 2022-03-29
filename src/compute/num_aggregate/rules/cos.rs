use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::math_op::cos::Cos;

pub(crate) fn cos_eval_rule(mut child: Vec<Expression>) -> Expression {
    let sub = child.pop().unwrap().num_aggregate();
    match &sub.root {
        AstNode::Operand(operand) => {
            if operand.is_zero() || operand.is_pi() {
                Expression::from(1_i64)
            } else {
                sub.cos()
            }
        }
        AstNode::Operator(operator) => match operator.descriptor {
            _ => sub.cos(),
        },
    }
}
