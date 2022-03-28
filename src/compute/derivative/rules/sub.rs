use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
};

pub(crate) fn sub_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError> {
    let r = child.pop().unwrap().derivative(to)?;
    let l = child.pop().unwrap().derivative(to)?;
    return Ok(l - r);
}
