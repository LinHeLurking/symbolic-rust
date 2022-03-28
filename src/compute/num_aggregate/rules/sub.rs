use crate::ast::ast_node::Expression;
use crate::compute::num_aggregate::NumAggregate;
use crate::smart_num::ToSmartNum;

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
