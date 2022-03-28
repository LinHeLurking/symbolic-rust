use crate::{
    ast::{tree::Expression, op::operand::Variable},
    compute::derivative::Derivative,
    math_op::cos::cos,
};

pub(crate) fn sin_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (sin(u))' = cos(u)*u'
    let sub = child.pop().unwrap();
    cos(sub.clone()) * sub.derivative(to)
}
