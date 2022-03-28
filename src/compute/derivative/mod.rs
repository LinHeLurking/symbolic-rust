mod rules;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::{AstOperand, ToVariable},
};

use self::rules::{
    add::add_derivative_rule, cos::cos_derivative_rule, div::div_derivative_rule,
    mul::mul_derivative_rule, neg::neg_derivative_rule, sin::sin_derivative_rule,
    sub::sub_derivative_rule,
};

use super::num_aggregate::NumAggregate;

pub(crate) trait Derivative<T: ToVariable> {
    type Output;
    fn derivative(self, to: T) -> Self::Output;
}

impl<T: ToVariable> Derivative<T> for Expression {
    type Output = Expression;

    fn derivative(self, to: T) -> Self::Output {
        let variable = to.to_variable().unwrap();
        match self.root {
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(_) => Expression::from(0_i64),
                AstOperand::Variable(another) => {
                    if variable.name == another.name {
                        Expression::from(1_i64)
                    } else {
                        Expression::from(0_i64)
                    }
                }
            },
            AstNode::Operator(operator) => {
                let child = self.child;
                if operator.descriptor == "Neg" {
                    neg_derivative_rule(child, &variable)
                } else if operator.descriptor == "Add" {
                    add_derivative_rule(child, &variable)
                } else if operator.descriptor == "Sub" {
                    sub_derivative_rule(child, &variable)
                } else if operator.descriptor == "Mul" {
                    mul_derivative_rule(child, &variable)
                } else if operator.descriptor == "Div" {
                    div_derivative_rule(child, &variable)
                } else if operator.descriptor == "Sin" {
                    sin_derivative_rule(child, &variable)
                } else if operator.descriptor == "Cos" {
                    cos_derivative_rule(child, &variable)
                } else {
                    panic!("Derivative not Implemented for {}", operator.symbol);
                }
            }
        }
        .num_aggregate()
    }
}

#[cfg(test)]
mod derivative_tests {
    use crate::{
        ast::ast_node::Expression,
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
            let y_d_x = y.derivative(x);
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
                let y_d_u = y.derivative(u);
                println!("{}", y_d_u);
                assert_eq!(y_d_u.to_string(), "sinx * (-sinu)");
            }
        }
        {
            let x = Expression::new_variable("x");
            let y = sin(x.clone()) * sin(x.clone()) * x.clone();
            println!("{}", y.derivative(x));
        }
    }
}