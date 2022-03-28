use std::ops::Mul;

use crate::{
    ast::{
        ast_node::{AstNode, Expression},
        op::{AstOperator, Variable},
    },
    compute::{derivative::Derivative, evaluate::NumericEvaluate},
    smart_num::ToSmartNum,
};

fn gen_op_mul() -> AstOperator {
    AstOperator {
        symbol: "*".to_string(),
        priority: 3_u32,
        descriptor: "Mul".to_string(),
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(gen_op_mul()),
            child: vec![self, rhs],
        }
    }
}

pub(crate) fn mul_eval_rule(mut child: Vec<Expression>) -> Expression {
    let r = child.pop().unwrap().eval();
    let l = child.pop().unwrap().eval();
    if l.is_num() && r.is_num() {
        Expression::from(l.to_smart_num().unwrap() * r.to_smart_num().unwrap())
    } else if l.is_zero() || r.is_zero() {
        Expression::from(0_i64)
    } else if l.is_one() {
        r
    } else if r.is_one() {
        l
    } else {
        l * r
    }
}

pub(crate) fn mul_derivative_rule(mut child: Vec<Expression>, to: &Variable) -> Expression {
    // (uv)' = u'v + uv'
    let v = child.pop().unwrap();
    let u = child.pop().unwrap();
    let v_d = v.clone().derivative(to);
    let u_d = u.clone().derivative(to);
    u_d * v + u * v_d
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast_node::Expression, compute::evaluate::NumericEvaluate};

    #[test]
    fn mul() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x * y;
            assert_eq!(z.to_string(), "x * y");
        }
        {
            let x = Expression::from(2_i32);
            let y = Expression::from(2_i32);
            let z = (x * y).eval();
            let expected = Expression::from(4_i32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
