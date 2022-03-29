use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
};

pub(crate) fn ln_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError<Expression>> {
    // ln(u)' = u'/u
    let u = child.pop().unwrap();
    return Ok(u.clone().derivative(to)? / u);
}
