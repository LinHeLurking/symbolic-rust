use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
    math_op::cos::cos,
};

pub(crate) fn sin_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError<Expression>> {
    // (sin(u))' = cos(u)*u'
    let sub = child.pop().unwrap();
    return Ok(cos(sub.clone()) * sub.derivative(to)?);
}
