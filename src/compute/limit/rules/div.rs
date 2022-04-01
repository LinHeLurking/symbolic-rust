use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::{
        derivative::Derivative,
        limit::{
            limit_expression::{LimitExpression, NoValidLimitFound},
            LimitTry,
        },
    },
};

pub(crate) trait DivLimitExprRule {
    type Output;
    fn div_limit_expr_rule(self, of: &Variable, to: Expression, order_try: u64) -> Self::Output;
}

impl DivLimitExprRule for Expression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn div_limit_expr_rule(
        mut self,
        of: &Variable,
        to: Expression,
        order_try: u64,
    ) -> Self::Output {
        let mut r = self.child.pop().unwrap();
        let mut l = self.child.pop().unwrap();
        // l/r
        for _ in 0..=order_try {
            let a = l.clone().limit(of, to.clone(), order_try)?;
            let b = r.clone().limit(of, to.clone(), order_try)?;
            let z = a / b;
            if z.is_ok() {
                return Ok(z.unwrap());
            }
            l = l.derivative(of).map_err(|_| NoValidLimitFound {})?;
            r = r.derivative(of).map_err(|_| NoValidLimitFound {})?;
        }
        return Err(NoValidLimitFound {});
    }
}

#[cfg(test)]
mod div_limit_tests {
    use crate::{ast::tree::Expression, compute::limit::LimitTry, math_op::sin::sin};

    #[test]
    fn sinx_by_x() {
        let x = Expression::new_variable("x");
        let y = sin(x.clone()) / x.clone();
        let expr_str = y.to_string();
        let limit = y.limit(x, Expression::zero(), 1).unwrap();
        println!("limit of {}, when x -> 0: {}", expr_str, limit);
        assert_eq!(limit.to_string(), "1");
    }

    #[test]
    fn sinx_by_x2() {
        let x = Expression::new_variable("x");
        let y = sin(x.clone()) / (x.clone() * x.clone());
        let expr_str = y.to_string();
        let limit = y.limit(x, Expression::zero(), 1).unwrap();
        println!("limit of {}, when x -> 0: {}", expr_str, limit);
        assert_eq!(limit.to_string(), "infinity");
    }

    #[test]
    fn sinx2_by_x() {
        let x = Expression::new_variable("x");
        let y = sin(x.clone()) * sin(x.clone()) / x.clone();
        let expr_str = y.to_string();
        let limit = y.limit(x, Expression::zero(), 1).unwrap();
        println!("limit of {}, when x -> 0: {}", expr_str, limit);
        assert_eq!(limit.to_string(), "0");
    }
}
