#![allow(dead_code)]

use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use self::{special_const::SpecialConst, val_holder::SmartNumVal};

pub mod rational;
pub mod special_const;
mod tests;
pub mod val_holder;

#[derive(Debug, Clone, Copy)]
pub struct SmartNum {
    value: SmartNumVal,
    tag: Option<SpecialConst>,
}

impl SmartNum {
    pub fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    pub fn to_f64(&self) -> f64 {
        self.value.to_f64()
    }

    pub fn new_rational(sign: i64, nominator: u64, denominator: u64) -> Option<SmartNum> {
        SmartNumVal::new_rational(sign, nominator, denominator)
            .and_then(|value| Some(SmartNum { value, tag: None }))
    }

    pub fn near(&self, other: &Self, eps: f64) -> bool {
        self.value.near(&other.value, eps)
    }

    pub fn one() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(1_i64),
            tag: Some(SpecialConst::One),
        }
    }

    pub fn zero() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(0_i64),
            tag: Some(SpecialConst::Zero),
        }
    }

    pub fn pi() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(PI),
            tag: Some(SpecialConst::Pi),
        }
    }

    pub fn e() -> SmartNum {
        SmartNum {
            value: SmartNumVal::from(E),
            tag: Some(SpecialConst::E),
        }
    }
}

impl Display for SmartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tag.is_some() {
            write!(f, "{}", self.tag.unwrap().to_string())
        } else {
            write!(f, "{}", self.value.to_string())
        }
    }
}

impl<T> From<T> for SmartNum
where
    T: Into<SmartNumVal>,
{
    fn from(v: T) -> Self {
        let value: SmartNumVal = v.into();
        let tag = None;
        SmartNum { value, tag }
    }
}

impl Neg for SmartNum {
    type Output = SmartNum;

    fn neg(self) -> Self::Output {
        SmartNum {
            value: -self.value,
            tag: self.tag,
        }
    }
}

impl Neg for &SmartNum {
    type Output = SmartNum;

    fn neg(self) -> Self::Output {
        SmartNum {
            value: -&self.value,
            tag: self.tag,
        }
    }
}

fn gen_tag(value: &SmartNumVal) -> Option<SpecialConst> {
    if !value.is_int() {
        None
    } else {
        let i = value.to_i64().unwrap();
        if i == 0 {
            Some(SpecialConst::Zero)
        } else if i == 1 {
            Some(SpecialConst::One)
        } else {
            None
        }
    }
}

impl Add for SmartNum {
    type Output = SmartNum;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Add for &SmartNum {
    type Output = SmartNum;

    fn add(self, rhs: Self) -> Self::Output {
        let value = &self.value + &rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Sub for SmartNum {
    type Output = SmartNum;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value - rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Sub for &SmartNum {
    type Output = SmartNum;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = &self.value - &rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Mul for SmartNum {
    type Output = SmartNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Mul for &SmartNum {
    type Output = SmartNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = &self.value * &rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Div for SmartNum {
    type Output = SmartNum;

    fn div(self, rhs: Self) -> Self::Output {
        let value = self.value / rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}

impl Div for &SmartNum {
    type Output = SmartNum;

    fn div(self, rhs: Self) -> Self::Output {
        let value = &self.value / &rhs.value;
        let tag = gen_tag(&value);
        SmartNum { value, tag }
    }
}
