use crate::{
    ast::{tree::Expression, op::operand::Variable},
    compute::derivative::Derivative,
};

pub(crate) fn div_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (u/v)' = (u'v - uv')/(v*v)
    let v = child.pop().unwrap();
    let u = child.pop().unwrap();
    let v_d = v.clone().derivative(to);
    let u_d = u.clone().derivative(to);
    (u_d * v.clone() - u * v_d) / (v.clone() * v)
}
