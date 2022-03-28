mod rules;

use std::{error::Error, fmt::Display};

use crate::ast::{
    op::{
        operand::{AstOperand, Variable},
        operator::OperatorType,
    },
    tree::{AstNode, Expression},
};

use self::rules::{
    add::add_derivative_rule, cos::cos_derivative_rule, div::div_derivative_rule,
    mul::mul_derivative_rule, neg::neg_derivative_rule, sin::sin_derivative_rule,
    sub::sub_derivative_rule,
};

use super::num_aggregate::NumAggregate;

pub(crate) trait Derivative<'a, T> {
    type Output;
    fn derivative(self, to: T) -> Self::Output;
}

#[derive(Debug)]
pub struct DerivativeError {}

impl Error for DerivativeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for DerivativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot be used as a variable")
    }
}

impl<'a, T: 'a> Derivative<'a, T> for Expression
where
    T: ToString + Display,
    Option<&'a Variable>: From<T>,
{
    type Output = Result<Expression, DerivativeError>;

    fn derivative(self, to_: T) -> Self::Output {
        let to = Into::<Option<&Variable>>::into(to_).ok_or(DerivativeError {})?;
        let result = match self.root {
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(_) => Expression::from(0_i64),
                AstOperand::Variable(another) => {
                    if to.name == another.name {
                        Expression::from(1_i64)
                    } else {
                        Expression::from(0_i64)
                    }
                }
            },
            AstNode::Operator(operator) => {
                let child = self.child;
                match operator.descriptor {
                    OperatorType::Neg => neg_derivative_rule(child, to)?,
                    OperatorType::Add => add_derivative_rule(child, to)?,
                    OperatorType::Sub => sub_derivative_rule(child, to)?,
                    OperatorType::Mul => mul_derivative_rule(child, to)?,
                    OperatorType::Div => div_derivative_rule(child, to)?,
                    OperatorType::Sin => sin_derivative_rule(child, to)?,
                    OperatorType::Cos => cos_derivative_rule(child, to)?,
                }
            }
        }
        .num_aggregate();
        Ok(result)
    }
}

#[cfg(test)]
mod derivative_tests {
    use crate::{
        ast::tree::Expression,
        compute::derivative::Derivative,
        math_op::{cos::cos, sin::sin},
    };

    #[test]
    fn d() {
        {
            let x = Expression::new_variable("x");
            let u = Expression::new_variable("u");
            let sin_x = sin(x.clone());
            let cos_u = cos(u.clone());
            let y = sin_x * cos_u;
            let y_d_x = y.derivative(&x).unwrap();
            println!("{}", y_d_x);
            assert_eq!(y_d_x.to_string(), "cosx * cosu");
        }
        {
            {
                let x = Expression::new_variable("x");
                let u = Expression::new_variable("u");
                let sin_x = sin(x.clone());
                let cos_u = cos(u.clone());
                let y = sin_x * cos_u;
                let y_d_u = y.derivative(&u).unwrap();
                println!("{}", y_d_u);
                assert_eq!(y_d_u.to_string(), "sinx * (-sinu)");
            }
        }
        {
            let x = Expression::new_variable("x");
            let y = sin(x.clone()) * sin(x.clone()) * x.clone();
            println!("{}", y.derivative(&x).unwrap());
        }
    }
}
