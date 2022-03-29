use crate::ast::op::operand::AstOperand;
use crate::ast::op::operator::OperatorType;
use crate::ast::tree::{AstNode, Expression};
use crate::compute::num_aggregate::NumAggregate;
use crate::math_op::ln::Ln;

pub(crate) fn ln_eval_rule(mut child: Vec<Expression>) -> Expression {
    let mut sub = child.pop().unwrap().num_aggregate();
    match &sub.root {
        AstNode::Operand(operand) => match operand {
            AstOperand::Variable(_) => sub.ln(),
            AstOperand::Num(v) => {
                if v.is_e() {
                    Expression::one()
                } else if v.is_one() {
                    Expression::zero()
                } else {
                    sub.ln()
                }
            }
        },
        AstNode::Operator(operator) => match operator.descriptor {
            OperatorType::Exp => sub.child.pop().unwrap(),
            _ => sub.ln(),
        },
    }
}

#[cfg(test)]
mod ln_eval_tests {
    use crate::{ast::tree::Expression, compute::num_aggregate::NumAggregate, math_op::{ln::ln, exp::exp}};

    #[test]
    fn ln_1() {
        let one = Expression::one();
        let ln_one = ln(one);
        assert!(ln_one.num_aggregate().is_zero());
    }

    #[test]
    fn ln_e() {
        let e = Expression::e();
        let ln_e = ln(e);
        assert!(ln_e.num_aggregate().is_one());
    }

    #[test]
    fn ln_exp() {
        let x = Expression::pi();
        let ln_exp_x = ln(exp(x));
        assert!(ln_exp_x.num_aggregate().is_pi());
    }
}
