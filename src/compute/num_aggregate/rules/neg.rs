use crate::ast::tree::Expression;
use crate::compute::num_aggregate::NumAggregate;
use crate::smart_num::ToSmartNum;

pub(crate) fn neg_eval_rule(mut child: Vec<Expression>) -> Expression {
    let sub = child.pop().unwrap().num_aggregate();
    if sub.is_num() {
        Expression::from(-sub.to_smart_num().unwrap())
    } else {
        -sub
    }
}
