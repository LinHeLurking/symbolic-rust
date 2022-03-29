use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
    math_op::exp::exp,
};

pub(crate) fn exp_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError<Expression>> {
    // exp(u)' = u'exp(u)
    let u = child.pop().unwrap();
    return Ok(u.clone().derivative(to)? * exp(u));
}
