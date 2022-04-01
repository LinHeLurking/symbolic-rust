#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::ast::tree::Expression;

/**
 * If we specify sign of infinity(infinitesimal), a few more
 * rules could be used to compute limit expression arithmetics.
 * But that's too difficult. I'll add those stuffs in the future.
*/

#[derive(Debug, Clone)]
pub enum LimitExpression {
    Infinitesimal,
    Infinity,
    BoundedFluctuation,
    Normal(Expression), // Non-zero expression
}

#[derive(Debug, Clone)]
pub struct NoValidLimitFound {}

impl Display for LimitExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitExpression::Infinitesimal => write!(f, "0"),
            LimitExpression::Infinity => write!(f, "infinity"),
            LimitExpression::BoundedFluctuation => write!(f, "~"),
            LimitExpression::Normal(r) => write!(f, "{}", r),
        }
    }
}

impl Neg for LimitExpression {
    type Output = LimitExpression;

    fn neg(self) -> Self::Output {
        match self {
            LimitExpression::Normal(r) => LimitExpression::Normal(-r),
            _ => self,
        }
    }
}

impl Add for LimitExpression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            LimitExpression::Infinitesimal => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinitesimal),
                LimitExpression::Infinity => Err(NoValidLimitFound {}),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Normal(r) => Ok(LimitExpression::Normal(r)),
            },
            LimitExpression::Infinity => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinity),
                LimitExpression::Infinity => Err(NoValidLimitFound {}),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::Infinity),
                LimitExpression::Normal(_) => Ok(LimitExpression::Infinity),
            },
            LimitExpression::BoundedFluctuation => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Infinity => Ok(LimitExpression::Infinity),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Normal(_) => Ok(LimitExpression::BoundedFluctuation),
            },
            LimitExpression::Normal(a) => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Normal(a)),
                LimitExpression::Infinity => Ok(LimitExpression::Infinity),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Normal(b) => Ok(LimitExpression::Normal(a + b)),
            },
        }
    }
}

impl Sub for LimitExpression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for LimitExpression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            LimitExpression::Infinitesimal => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinitesimal),
                LimitExpression::Infinity => Err(NoValidLimitFound {}),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::Infinitesimal),
                LimitExpression::Normal(_) => Ok(LimitExpression::Infinitesimal),
            },
            LimitExpression::Infinity => match rhs {
                LimitExpression::Infinitesimal => Err(NoValidLimitFound {}),
                LimitExpression::Infinity => Ok(LimitExpression::Infinity),
                LimitExpression::BoundedFluctuation => Err(NoValidLimitFound {}), // x * sinx should be infinity of unknown?
                LimitExpression::Normal(_) => Ok(LimitExpression::Infinity),
            },
            LimitExpression::BoundedFluctuation => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinitesimal),
                LimitExpression::Infinity => Err(NoValidLimitFound {}),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Normal(_) => Ok(LimitExpression::BoundedFluctuation),
            },
            LimitExpression::Normal(a) => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinitesimal),
                LimitExpression::Infinity => Ok(LimitExpression::Infinity),
                LimitExpression::BoundedFluctuation => Ok(LimitExpression::BoundedFluctuation),
                LimitExpression::Normal(b) => Ok(LimitExpression::Normal(a * b)),
            },
        }
    }
}

// TODO: maybe expression simplifications are needed before evaluating if x.is_zero()

impl Div for LimitExpression {
    type Output = Result<LimitExpression, NoValidLimitFound>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            LimitExpression::Infinitesimal => match rhs {
                LimitExpression::Infinitesimal => Err(NoValidLimitFound {}),
                LimitExpression::Infinity => Ok(LimitExpression::Infinitesimal),
                LimitExpression::BoundedFluctuation => Err(NoValidLimitFound {}), // x/sinx
                LimitExpression::Normal(r) => {
                    if r.is_zero() {
                        Err(NoValidLimitFound {})
                    } else {
                        Ok(LimitExpression::Infinitesimal)
                    }
                }
            },
            LimitExpression::Infinity => match rhs {
                LimitExpression::Infinitesimal => Ok(LimitExpression::Infinity),
                LimitExpression::Infinity => Err(NoValidLimitFound {}),
                LimitExpression::BoundedFluctuation => Err(NoValidLimitFound {}),
                LimitExpression::Normal(_) => Ok(LimitExpression::Infinity),
            },
            LimitExpression::BoundedFluctuation => match rhs {
                LimitExpression::Infinitesimal => Err(NoValidLimitFound {}),
                LimitExpression::Infinity => Ok(LimitExpression::Infinitesimal),
                LimitExpression::BoundedFluctuation => Err(NoValidLimitFound {}),
                LimitExpression::Normal(r) => {
                    if r.is_zero() {
                        Err(NoValidLimitFound {})
                    } else {
                        Ok(LimitExpression::BoundedFluctuation)
                    }
                }
            },
            LimitExpression::Normal(a) => match rhs {
                LimitExpression::Infinitesimal => {
                    if !a.is_zero() {
                        Ok(LimitExpression::Infinity)
                    } else {
                        Err(NoValidLimitFound {})
                    }
                }
                LimitExpression::Infinity => Ok(LimitExpression::Infinitesimal),
                LimitExpression::BoundedFluctuation => Err(NoValidLimitFound {}),
                LimitExpression::Normal(b) => Ok(LimitExpression::Normal(a / b)),
            },
        }
    }
}
