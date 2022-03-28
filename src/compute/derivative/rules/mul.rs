use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
};

pub(crate) fn mul_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError> {
    // (uv)' = u'v + uv'
    let v = child.pop().unwrap();
    let u = child.pop().unwrap();
    let v_d = v.clone().derivative(to)?;
    let u_d = u.clone().derivative(to)?;
    return Ok(u_d * v + u * v_d);
}
