use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
};

pub(crate) fn add_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError<Expression>> {
    let r = child.pop().unwrap().derivative(to)?;
    let l = child.pop().unwrap().derivative(to)?;
    return Ok(l + r);
}
