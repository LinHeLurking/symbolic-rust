use crate::ast::op::operand::AstOperand;
use crate::ast::op::operator::OperatorType;
use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::math_op::pow::Pow;

pub(crate) fn pow_eval_rule(mut child: Vec<Expression>) -> Expression {
    let r = child.pop().unwrap().num_aggregate();
    let mut sub = child.pop().unwrap().num_aggregate();
    match &sub.root {
        AstNode::Operand(operand) => match operand {
            AstOperand::Variable(_) => sub.pow(r),
            AstOperand::Num(v) => {
                if v.is_zero() {
                    Expression::from(1_i64)
                } else if v.is_one() {
                    r
                } else {
                    sub.pow(r)
                }
            }
        },
        AstNode::Operator(operator) => match operator.descriptor {
            OperatorType::Ln => sub.child.pop().unwrap(),
            _ => sub.pow(r),
        },
    }
}
