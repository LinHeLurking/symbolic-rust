use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::limit::limit_expression::LimitExpression,
};

pub(crate) trait NegLimitExprRule {
    type Output;
    fn neg_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

impl NegLimitExprRule for Expression {
    type Output = LimitExpression;

    fn neg_limit_expr_rule(mut self, of: &Variable, to: Expression, order_try: u64) -> Self::Output {
        let sub = self.child.pop().unwrap();
        -sub.neg_limit_expr_rule(of, to, order_try)
    }
}
