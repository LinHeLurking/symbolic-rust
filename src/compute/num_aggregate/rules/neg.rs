use crate::ast::op::operator::OperatorType;
use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::smart_num::ToSmartNum;

pub(crate) fn neg_eval_rule(mut child: Vec<Expression>) -> Expression {
    let mut sub = child.pop().unwrap().num_aggregate();
    if sub.is_num() {
        Expression::from(-sub.to_smart_num().unwrap())
    } else {
        match &sub.root {
            AstNode::Operator(operator) => match &operator.descriptor {
                OperatorType::Neg => sub.child.pop().unwrap(),
                _ => -sub,
            },
            AstNode::Operand(_) => -sub,
        }
    }
}
