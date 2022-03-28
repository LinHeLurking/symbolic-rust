use crate::{
    ast::{tree::Expression, op::operand::Variable},
    compute::derivative::Derivative,
    math_op::sin::sin,
};

pub(crate) fn cos_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (cos(u))' = -sin(u)*u'
    let sub = child.pop().unwrap();
    -sin(sub.clone()) * sub.derivative(to)
}
