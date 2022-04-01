use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::limit::{
        limit_expression::{LimitExpression, NoValidLimitFound},
        LimitTry,
    },
};

pub(crate) trait SubLimitExprRule {
    type Output;
    fn sub_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

impl SubLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn sub_limit_expr_rule(
        mut self,
        of: &Variable,
        to: Expression,
        order_try: u64,
    ) -> Self::Output {
        let r = self.child.pop().unwrap();
        let l = self.child.pop().unwrap();
        (l + (-r)).limit(of, to, order_try)
    }
}
