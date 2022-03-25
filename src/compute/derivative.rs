use super::simplify::CanSimplify;
use crate::ast::{
    ast_node::{AstNode, Expression},
    op::{AstOperand, AstOperator, Variable},
    smart_num::SmartNum,
};

pub trait CanDerivative {
    type Output;
    fn derivative(&self, x: &Variable) -> Self::Output;
}

impl CanDerivative for Expression {
    type Output = Expression;
    fn derivative(&self, x: &Variable) -> Self::Output {
        let mut expr = match &self.me {
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(_) => Expression::from(0_i64),
                AstOperand::Variable(y) => {
                    if y.name == x.name {
                        Expression::from(1_i64)
                    } else {
                        Expression::from(0_i64)
                    }
                }
            },
            AstNode::Operator(op) => match op {
                AstOperator::Neg(_) => -self.child[0].derivative(x),
                AstOperator::Add(_) => self.child[0].derivative(x) + self.child[1].derivative(x),
                AstOperator::Sub(_) => self.child[0].derivative(x) - self.child[1].derivative(x),
                AstOperator::Mul(_) => {
                    let l = &self.child[0];
                    let r = &self.child[1];
                    l.clone() * r.derivative(x) + l.derivative(x) * r.clone()
                }
                AstOperator::Div(_) => {
                    // (u/v)' = u'v-vu'/(v*v)
                    let u = &self.child[0];
                    let v = &self.child[1];
                    (u.derivative(x) * v.clone() - u.clone() * v.derivative(x))
                        / (v.clone() * v.clone())
                }
            },
        };
        expr.simplify();
        return expr;
    }
}

#[test]
fn derivative_constant() {
    {
        let a = Expression::from(1_f32);
        let b = Expression::from(2_u32);
        let c = Expression::from(3_i64);
        let d = Expression::from(4_f64);
        let x = (a + b) * (c - d);
        let y = x.clone().derivative(&Variable::new_variable("x"));
        assert!(y.is_num());
        assert!(y.to_smart_num().near(SmartNum::from(0_i64), 1e-9));
    }
}

#[test]
fn derivative_variable() {
    {
        let x = Variable::new_variable("x");
        let f = Expression::new_variable("x");
        let y = f.derivative(&x);
        assert!(y.is_num());
        assert!(y.to_smart_num().near(SmartNum::one(), 1e-9));
    }
}
