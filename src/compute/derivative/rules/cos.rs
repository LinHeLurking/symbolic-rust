use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
    math_op::sin::sin,
};

pub(crate) fn cos_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError> {
    // (cos(u))' = -sin(u)*u'
    let sub = child.pop().unwrap();
    return Ok(-sin(sub.clone()) * sub.derivative(to)?);
}
