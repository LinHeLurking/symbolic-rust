use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
    math_op::{ln::Ln, pow::Pow},
};

pub(crate) fn pow_derivative_rule(
    mut child: Vec<Expression>,
    to: &Variable,
) -> Result<Expression, DerivativeError<Expression>> {
    // (u^r)' = (exp(r*ln(u)))' = (r*ln(u))' * exp(r*ln(u)) = u^r * (r' * ln(u) + r * u' / u)
    let r = child.pop().unwrap();
    let u = child.pop().unwrap();
    let r_d = r.clone().derivative(to)?;
    let u_d = u.clone().derivative(to)?;
    let ln_u = u.clone().ln();
    let u_pow_r = u.clone().pow(r.clone());
    return Ok(u_pow_r * (r_d * ln_u + r * u_d / u));
}
