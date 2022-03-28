#![allow(dead_code)]

use std::{error::Error, fmt::Display};

use crate::{
    ast::{op::operand::Variable, tree::Expression},
    compute::derivative::{Derivative, DerivativeError},
    smart_num::SmartNum,
};

pub struct PartialExpansion {
    order: u32,
    coefficient: Vec<SmartNum>,
    residual: Expression,
}

trait TaylorExpansion<'a, T, U> {
    type Output;
    fn taylor_expansion(self, of: U, at: &T) -> Self::Output;
}

#[derive(Debug)]
struct TaylorExpansionError<'a> {
    source: Option<&'a dyn Error>,
}

impl Display for TaylorExpansionError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Expansion error")
    }
}

impl<'a, T, U> TaylorExpansion<'a, T, U> for Expression
where
    U: Into<Option<&'a Variable>>,
    T: Into<Expression>,
{
    type Output = Result<PartialExpansion, TaylorExpansionError<'a>>;

    fn taylor_expansion(self, of: U, at: &T) -> Self::Output {
        todo!()
    }
}
