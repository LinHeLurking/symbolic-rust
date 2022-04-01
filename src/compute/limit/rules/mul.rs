use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::limit::{
        limit_expression::{LimitExpression, NoValidLimitFound},
        LimitTry,
    },
};

pub(crate) trait MulLimitExprRule {
    type Output;
    fn mul_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

impl MulLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn mul_limit_expr_rule(
        mut self,
        of: &Variable,
        to: Expression,
        order_try: u64,
    ) -> Self::Output {
        let r_ = self.child.pop().unwrap();
        let l_ = self.child.pop().unwrap();
        let r_limit = r_.clone().limit(of, to.clone(), order_try)?;
        let l_limit = l_.clone().limit(of, to.clone(), order_try)?;
        let r_lim_is_zero = match &r_limit {
            LimitExpression::Infinitesimal => true,
            LimitExpression::Infinity => false,
            LimitExpression::BoundedFluctuation => false,
            LimitExpression::Normal(r) => r.is_zero(),
        };
        let first_try = l_limit * r_limit;
        if first_try.is_ok() {
            return Ok(first_try.unwrap());
        } else {
            if r_lim_is_zero {
                // (l * r) = r / (1/l) if l is not close 0
                let z = r_ / (Expression::one() / l_);
                return z.limit(of, to, order_try);
            } else {
                // (l * r) = l / (1/r) if r is not close 0
                let z = l_ / (Expression::one() / r_);
                return z.limit(of, to, order_try);
            }
        }
    }
}
