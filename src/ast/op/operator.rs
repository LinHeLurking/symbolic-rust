#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatorType {
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Sin,
    Cos,
    Exp,
    Ln,
    Pow,
}

#[derive(Debug, Clone)]
pub struct AstOperator {
    pub symbol: String,
    pub priority: u32,
    pub(crate) descriptor: OperatorType,
}

impl PartialEq for AstOperator {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl Eq for AstOperator {}

impl Display for AstOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
