use crate::{
    ast::{ast_node::Expression, op::Variable},
    compute::derivative::Derivative,
};

pub(crate) fn mul_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (uv)' = u'v + uv'
    let v = child.pop().unwrap();
    let u = child.pop().unwrap();
    let v_d = v.clone().derivative(to);
    let u_d = u.clone().derivative(to);
    u_d * v + u * v_d
}
