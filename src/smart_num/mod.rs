#![allow(dead_code)]

use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use self::{
    rational::{RationalNum, ToRational},
    special_const::ConstType,
    val_holder::{IsClose, SmartNumVal},
};

pub mod rational;
mod special_const;
mod tests;
pub(super) mod val_holder;

#[derive(Debug, Clone, Copy)]
pub struct SmartNum {
    value: SmartNumVal,
    tag: ConstType,
}

pub trait ToSmartNum {
    type Output;
    fn to_smart_num(self) -> Self::Output;
}

impl SmartNum {
    pub fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    pub fn to_f64(&self) -> f64 {
        self.value.to_f64()
    }

    pub fn new_rational(sign: i64, nominator: u64, denominator: u64) -> Option<SmartNum> {
        SmartNumVal::new_rational(sign, nominator, denominator).and_then(|value| {
            Some(SmartNum {
                value,
                tag: if value.is_zero() {
                    ConstType::Zero
                } else if value.is_one() {
                    ConstType::One
                } else {
                    ConstType::Nothing
                },
            })
        })
    }

    pub fn one() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(1_i64),
            tag: ConstType::One,
        }
    }

    pub fn is_one(&self) -> bool {
        self.tag == ConstType::One
    }

    pub fn zero() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(0_i64),
            tag: ConstType::Zero,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.tag == ConstType::Zero
    }

    pub fn pi() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(PI),
            tag: ConstType::Pi,
        }
    }

    pub fn is_pi(&self) -> bool {
        self.tag == ConstType::Pi
    }

    pub fn e() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(E),
            tag: ConstType::E,
        }
    }

    pub fn is_e(&self) -> bool {
        self.tag == ConstType::E
    }
}

impl ToRational for SmartNum {
    type Output = Option<RationalNum>;

    fn to_rational(self) -> Self::Output {
        match self.value {
            SmartNumVal::Integer(v) => Some(RationalNum::from(v)),
            SmartNumVal::Rational(v) => Some(v),
            SmartNumVal::Real(_) => None,
        }
    }
}

impl<'a> ToRational for &'a SmartNum {
    type Output = Option<RationalNum>;

    fn to_rational(self) -> Self::Output {
        self.clone().to_rational()
    }
}

impl Display for SmartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tag != ConstType::Nothing {
            write!(f, "{}", self.tag.to_string())
        } else {
            write!(f, "{}", self.value.to_string())
        }
    }
}

impl IsClose<SmartNum> for SmartNum {
    fn is_close(self, other: SmartNum, eps: f64) -> bool {
        self.value.is_close(other.value, eps)
    }
}

impl IsClose<&SmartNum> for &SmartNum {
    fn is_close(self, other: &SmartNum, eps: f64) -> bool {
        (&self.value).is_close(&other.value, eps)
    }
}

impl<T> From<T> for SmartNum
where
    T: Into<SmartNumVal>,
{
    fn from(v: T) -> Self {
        let value: SmartNumVal = v.into();
        let tag = if value.is_one() {
            ConstType::One
        } else if value.is_zero() {
            ConstType::Zero
        } else {
            ConstType::Nothing
        };
        SmartNum { value, tag }
    }
}

fn wrap_val(value: SmartNumVal) -> SmartNum {
    if value.is_zero() {
        SmartNum::zero()
    } else if value.is_one() {
        SmartNum::one()
    } else {
        SmartNum {
            value,
            tag: ConstType::Nothing,
        }
    }
}

impl Neg for SmartNum {
    type Output = SmartNum;

    fn neg(self) -> Self::Output {
        wrap_val(-self.value)
    }
}

impl Add for SmartNum {
    type Output = SmartNum;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            rhs
        } else if rhs.is_zero() {
            self
        } else {
            wrap_val(self.value + rhs.value)
        }
    }
}

impl Sub for SmartNum {
    type Output = SmartNum;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            -rhs
        } else if rhs.is_zero() {
            self
        } else {
            wrap_val(self.value - rhs.value)
        }
    }
}

impl Mul for SmartNum {
    type Output = SmartNum;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_one() {
            rhs
        } else if rhs.is_one() {
            self
        } else {
            wrap_val(self.value * rhs.value)
        }
    }
}

impl Div for SmartNum {
    type Output = SmartNum;

    fn div(self, rhs: Self) -> Self::Output {
        wrap_val(self.value / rhs.value)
    }
}
