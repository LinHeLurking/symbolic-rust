use crate::{
    ast::{ast_node::Expression, op::Variable},
    compute::derivative::Derivative,
};

pub(crate) fn sub_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    let r = child.pop().unwrap().derivative(to);
    let l = child.pop().unwrap().derivative(to);
    l - r
}
