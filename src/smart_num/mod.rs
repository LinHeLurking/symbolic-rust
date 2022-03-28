#![allow(dead_code)]

use std::{
    f64::consts::{E, PI},
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use self::{
    special_const::ConstType,
    val_holder::{IsClose, SmartNumVal},
};

pub mod rational;
pub mod special_const;
mod tests;
pub mod val_holder;

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
                tag: ConstType::Nothing,
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
        let tag = gen_tag(&value);
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

fn gen_tag(value: &SmartNumVal) -> ConstType {
    if !value.is_int() {
        ConstType::Nothing
    } else {
        let i = value.to_i64().unwrap();
        if i == 0 {
            ConstType::Zero
        } else if i == 1 {
            ConstType::One
        } else {
            ConstType::Nothing
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
