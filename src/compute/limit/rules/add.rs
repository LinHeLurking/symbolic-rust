use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::limit::{
        limit_expression::{LimitExpression, NoValidLimitFound},
        LimitTry,
    },
};

pub(crate) trait AddLimitExprRule {
    type Output;
    fn add_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

impl AddLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn add_limit_expr_rule(
        mut self,
        of: &Variable,
        to: Expression,
        order_try: u64,
    ) -> Self::Output {
        let r_ = self.child.pop().unwrap();
        let l_ = self.child.pop().unwrap();
        let r_limit = r_.clone().limit(of, to.clone(), order_try)?;
        let l_limit = l_.clone().limit(of, to.clone(), order_try)?;
        let first_try = l_limit + r_limit;
        if first_try.is_ok() {
            return Ok(first_try.unwrap());
        } else {
            // l + r = (l/r + 1)/(1/r)
            let z = (l_ / r_.clone() + Expression::one()) / (Expression::one() / r_);
            return z.limit(of, to, order_try);
        }
    }
}
