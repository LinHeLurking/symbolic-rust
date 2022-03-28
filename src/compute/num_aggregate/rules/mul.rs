use crate::ast::tree::Expression;
use crate::compute::num_aggregate::NumAggregate;
use crate::smart_num::ToSmartNum;

pub(crate) fn mul_eval_rule(mut child: Vec<Expression>) -> Expression {
    let r = child.pop().unwrap().num_aggregate();
    let l = child.pop().unwrap().num_aggregate();
    if l.is_num() && r.is_num() {
        Expression::from(l.to_smart_num().unwrap() * r.to_smart_num().unwrap())
    } else if l.is_zero() || r.is_zero() {
        Expression::from(0_i64)
    } else if l.is_one() {
        r
    } else if r.is_one() {
        l
    } else {
        l * r
    }
}