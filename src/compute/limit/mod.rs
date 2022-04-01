use crate::{
    ast::{
        op::{
            operand::{AstOperand, Variable},
            operator::OperatorType,
        },
        tree::{AstNode, Expression},
    },
    compute::num_aggregate::NumAggregate,
};

use self::{
    limit_expression::{LimitExpression, NoValidLimitFound},
    rules::{
        add::AddLimitExprRule, div::DivLimitExprRule, mul::MulLimitExprRule, neg::NegLimitExprRule,
        sub::SubLimitExprRule, sin::SinLimitExprRule, cos::CosLimitExprRule,
    },
};

pub mod limit_expression;
mod rules;

trait LimitTry<T, U> {
    type Output;
    fn limit(self, of: T, to: U, order_try: u64) -> Self::Output;
}

impl<'a> LimitTry<&'a Variable, Expression> for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn limit(self, of: &'a Variable, to: Expression, order_try: u64) -> Self::Output {
        let z = match &self.root {
            AstNode::Operand(operand) => match operand {
                AstOperand::Num(num) => Ok(LimitExpression::Normal(Expression::from(num.clone()))),
                AstOperand::Variable(variable) => {
                    if variable.name == of.name {
                        if to.is_zero() {
                            Ok(LimitExpression::Infinitesimal)
                        } else {
                            Ok(LimitExpression::Normal(to))
                        }
                    } else {
                        Ok(LimitExpression::Normal(Expression::from(variable.clone())))
                    }
                }
            },
            AstNode::Operator(operator) => match operator.descriptor {
                OperatorType::Neg => Ok(self.neg_limit_expr_rule(of, to, order_try)),
                OperatorType::Add => self.add_limit_expr_rule(of, to, order_try),
                OperatorType::Sub => self.sub_limit_expr_rule(of, to, order_try),
                OperatorType::Mul => self.mul_limit_expr_rule(of, to, order_try),
                OperatorType::Div => self.div_limit_expr_rule(of, to, order_try),
                OperatorType::Sin => self.sin_limit_expr_rule(of, to, order_try),
                OperatorType::Cos => self.cos_limit_expr_rule(of, to, order_try),
                OperatorType::Exp => todo!(),
                OperatorType::Ln => todo!(),
                OperatorType::Pow => todo!(),
            },
        }?;
        let flag = match &z {
            LimitExpression::Normal(_) => true,
            _ => false,
        };
        return if flag {
            match z {
                LimitExpression::Normal(expr) => Ok(LimitExpression::Normal(expr.num_aggregate())),
                _ => Err(NoValidLimitFound {}), // Unreachable
            }
        } else {
            Ok(z)
        };
    }
}

impl LimitTry<Expression, Expression> for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn limit(self, of_: Expression, to: Expression, order_try: u64) -> Self::Output {
        let of_raw: Option<Variable> = of_.into();
        let of = of_raw.unwrap();
        self.limit(&of, to, order_try)
    }
}
