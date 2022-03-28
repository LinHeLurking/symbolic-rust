use crate::{
    ast::{tree::Expression, op::operand::Variable},
    compute::derivative::Derivative,
};

pub(crate) fn neg_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    let sub = child.pop().unwrap().derivative(to);
    -sub
}
