use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::{
        limit::limit_expression::{LimitExpression, NoValidLimitFound},
        num_aggregate::NumAggregate,
        substitute::Substitute,
    },
};

pub(crate) trait CosLimitExprRule {
    type Output;
    fn cos_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

// TODO: cos(k * pi + pi / 2) = 0

impl CosLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn cos_limit_expr_rule(self, of: &Variable, to: Expression, _order_try: u64) -> Self::Output {
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
