use crate::{
    ast::{tree::Expression, op::operand::Variable},
    compute::derivative::Derivative,
};

pub(crate) fn sub_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    let r = child.pop().unwrap().derivative(to);
    let l = child.pop().unwrap().derivative(to);
    l - r
}
