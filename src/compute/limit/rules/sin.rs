use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::{
        limit::limit_expression::{LimitExpression, NoValidLimitFound},
        num_aggregate::NumAggregate,
        substitute::Substitute,
    },
};

pub(crate) trait SinLimitExprRule {
    type Output;
    fn sin_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

// TODO: sin(k * pi) = 0

impl SinLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn sin_limit_expr_rule(self, of: &Variable, to: Expression, _order_try: u64) -> Self::Output {
        if to.is_zero() {
            Ok(LimitExpression::Infinitesimal)
        } else {
            self.substitute(of, &to)
                .map_err(|_| NoValidLimitFound {})
                .map(|expr| {
                    let z = expr.num_aggregate();
                    if z.is_zero() {
                        LimitExpression::Infinitesimal
                    } else {
                        LimitExpression::Normal(z)
                    }
                })
        }
    }
}
