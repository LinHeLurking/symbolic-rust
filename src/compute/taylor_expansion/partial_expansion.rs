#![allow(dead_code)]

use std::{fmt::Display, vec};

use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::{derivative::Derivative, num_aggregate::NumAggregate, substitute::Substitute},
    smart_num::SmartNum,
};

#[derive(Debug, Clone)]
pub struct PartialExpansion {
    order: u32,
    of: Expression,
    coefficient: Vec<Expression>,
    residual: Expression,
}

impl Display for PartialExpansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut content = String::from("");
        for (idx, term) in self.coefficient.iter().enumerate() {
            if term.is_zero() {
                continue;
            }
            content.push_str(format!("{} * ", term).as_str());
            content.push_str(format!("{}^{}", self.of, idx).as_str());
            content.push_str(" + ")
        }
        content.push_str(format!("O({}^{})", self.of, self.order + 1).as_str());
        write!(f, "{}", content)
    }
}

trait TaylorExpansion<'a, T, U> {
    type Output;
    fn taylor_expansion(self, of: T, at: U, order: u32) -> Self::Output;
}

#[derive(Debug)]
struct TaylorExpansionError<T = Expression> {
    err_expr: T,
    reason: &'static str,
}

impl<T> Display for TaylorExpansionError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot expand {} due to {}", self.err_expr, self.reason)
    }
}

impl<'a> TaylorExpansion<'a, &Variable, &Expression> for Expression {
    type Output = Result<PartialExpansion, TaylorExpansionError<Expression>>;

    fn taylor_expansion(self, of: &Variable, at: &Expression, order: u32) -> Self::Output {
        let mut coefficient: Vec<Expression> = vec![];
        let mut residual = self;
        let mut factorial = SmartNum::from(1_i64);
        let err_expr = residual.clone();
        for k in 0..=order {
            factorial = factorial * SmartNum::from(k.max(1_u32));
            let cur = residual
                .clone()
                .substitute(of, at)
                .map_err(|_| TaylorExpansionError {
                    err_expr: err_expr.clone(),
                    reason: "substitute failure",
                })?
                .num_aggregate()
                / Expression::from(factorial);
            coefficient.push(cur.num_aggregate());

            residual = residual.derivative(of).map_err(|_| TaylorExpansionError {
                err_expr: err_expr.clone(),
                reason: "derivative failure",
            })?;
        }
        return Ok(PartialExpansion {
            order,
            of: Expression::from(of.clone()),
            coefficient,
            residual,
        });
    }
}

impl<'a> TaylorExpansion<'a, Expression, Expression> for Expression {
    type Output = Result<PartialExpansion, TaylorExpansionError<Expression>>;

    fn taylor_expansion(self, of: Expression, at: Expression, order: u32) -> Self::Output {
        let of_raw = Into::<Option<Variable>>::into(of.clone());
        if of_raw.is_none() {
            return Err(TaylorExpansionError {
                err_expr: of,
                reason: "not a variable",
            });
        } else {
            let of_ = of_raw.unwrap();
            return self.taylor_expansion(&of_, &at, order);
        }
    }
}

#[cfg(test)]
mod taylor_expansion_test {
    use crate::{ast::tree::Expression, math_op::sin::sin};

    use super::TaylorExpansion;

    #[test]
    fn expand_sin() {
        let x = Expression::new_variable("x");
        let y = sin(x.clone());
        let zero = Expression::from(0_i64);
        println!("Taylor series of {}:", y);
        let expansion = y.taylor_expansion(x, zero, 10).unwrap();
        println!("{}", expansion);
    }
}
