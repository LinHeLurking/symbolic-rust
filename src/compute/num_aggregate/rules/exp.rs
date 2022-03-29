use crate::ast::op::operand::AstOperand;
use crate::ast::op::operator::OperatorType;
use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::math_op::exp::Exp;

pub(crate) fn exp_eval_rule(mut child: Vec<Expression>) -> Expression {
    let mut sub = child.pop().unwrap().num_aggregate();
    match &sub.root {
        AstNode::Operand(operand) => match operand {
            AstOperand::Variable(_) => sub.exp(),
            AstOperand::Num(v) => {
                if v.is_zero() {
                    Expression::from(1_i64)
                } else if v.is_one() {
                    Expression::e()
                } else {
                    sub.exp()
                }
            }
        },
        AstNode::Operator(operator) => match operator.descriptor {
            OperatorType::Ln => sub.child.pop().unwrap(),
            _ => sub.exp(),
        },
    }
}

#[cfg(test)]
mod exp_eval_tests {
    use crate::{ast::tree::Expression, compute::num_aggregate::NumAggregate, math_op::{exp::exp, ln::ln}};

    #[test]
    fn exp_0() {
        let zero = Expression::zero();
        let v = exp(zero);
        assert!(v.num_aggregate().is_one());
    }
    #[test]
    fn exp_1() {
        let one = Expression::one();
        let v = exp(one);
        assert!(v.num_aggregate().is_e());
    }

    #[test]
    fn exp_ln() {
        let x = Expression::pi();
        let exp_ln_x = exp(ln(x));
        assert!(exp_ln_x.num_aggregate().is_pi());
    }
}
