#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::rational::{RationalNum, ToRational};

#[derive(Debug, Clone, Copy)]
pub(super) enum SmartNumVal {
    Integer(i64),
    Rational(RationalNum),
    Real(f64),
}

pub trait IsClose<RHS = Self> {
    fn is_close(self, rhs: RHS, eps: f64) -> bool;
}

impl Display for SmartNumVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            SmartNumVal::Integer(v) => format!("{}", v),
            SmartNumVal::Rational(v) => format!("{}", v),
            SmartNumVal::Real(v) => format!("{:.3}", v),
        };
        write!(f, "{}", content)
    }
}

impl SmartNumVal {
    pub fn is_int(&self) -> bool {
        match self {
            SmartNumVal::Integer(_) => true,
            SmartNumVal::Rational(_) => false,
            SmartNumVal::Real(_) => false,
        }
    }

    pub fn is_rational(&self) -> bool {
        match self {
            SmartNumVal::Integer(_) => false,
            SmartNumVal::Rational(_) => true,
            SmartNumVal::Real(_) => false,
        }
    }

    pub fn is_real(&self) -> bool {
        match self {
            SmartNumVal::Integer(_) => false,
            SmartNumVal::Rational(_) => false,
            SmartNumVal::Real(_) => true,
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            SmartNumVal::Integer(v) => *v == 0,
            SmartNumVal::Rational(v) => v.nominator == 0 && v.denominator != 0,
            SmartNumVal::Real(_) => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            SmartNumVal::Integer(v) => *v == 1,
            SmartNumVal::Rational(v) => {
                v.sign == 1 && v.nominator == v.denominator && v.denominator != 0
            }
            SmartNumVal::Real(_) => false,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            SmartNumVal::Integer(v) => Some(*v),
            SmartNumVal::Rational(_) => None,
            SmartNumVal::Real(_) => None,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            SmartNumVal::Real(v) => *v,
            SmartNumVal::Rational(v) => v.to_f64(),
            SmartNumVal::Integer(v) => *v as f64,
        }
    }

    pub fn new_rational(sign: i64, nominator: u64, denominator: u64) -> Option<SmartNumVal> {
        RationalNum::new(sign, nominator, denominator).and_then(|v| Some(SmartNumVal::Rational(v)))
    }

    pub fn zero() -> SmartNumVal {
        SmartNumVal::from(0_i64)
    }

    pub fn one() -> SmartNumVal {
        SmartNumVal::from(1_i64)
    }
}

impl IsClose<SmartNumVal> for SmartNumVal {
    fn is_close(self, rhs: SmartNumVal, eps: f64) -> bool {
        return (self.to_f64() - rhs.to_f64()).abs() < eps;
    }
}

impl IsClose<&SmartNumVal> for &SmartNumVal {
    fn is_close(self, rhs: &SmartNumVal, eps: f64) -> bool {
        return (self.to_f64() - rhs.to_f64()).abs() < eps;
    }
}

impl ToRational for SmartNumVal {
    type Output = Option<RationalNum>;

    fn to_rational(self) -> Self::Output {
        match self {
            SmartNumVal::Integer(_) => None,
            SmartNumVal::Rational(v) => Some(v),
            SmartNumVal::Real(_) => None,
        }
    }
}

impl<'a> ToRational for &'a SmartNumVal {
    type Output = Option<&'a RationalNum>;

    fn to_rational(self) -> Self::Output {
        match self {
            SmartNumVal::Integer(_) => None,
            SmartNumVal::Rational(v) => Some(v),
            SmartNumVal::Real(_) => None,
        }
    }
}

impl From<i64> for SmartNumVal {
    fn from(v: i64) -> Self {
        SmartNumVal::Integer(v)
    }
}

impl From<f64> for SmartNumVal {
    fn from(v: f64) -> Self {
        SmartNumVal::Real(v)
    }
}

impl From<f32> for SmartNumVal {
    fn from(v: f32) -> Self {
        SmartNumVal::Real(v as f64)
    }
}

// Automatically generated codes for other From<T>
impl From<u8> for SmartNumVal {
    fn from(v: u8) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<i8> for SmartNumVal {
    fn from(v: i8) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<u16> for SmartNumVal {
    fn from(v: u16) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<i16> for SmartNumVal {
    fn from(v: i16) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<u32> for SmartNumVal {
    fn from(v: u32) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<i32> for SmartNumVal {
    fn from(v: i32) -> Self {
        SmartNumVal::Integer(i64::from(v))
    }
}

impl From<u64> for SmartNumVal {
    fn from(v: u64) -> Self {
        SmartNumVal::Integer(v as i64)
    }
}

impl Add for SmartNumVal {
    type Output = SmartNumVal;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            SmartNumVal::Integer(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Integer(i + j),
                SmartNumVal::Rational(j) => SmartNumVal::Rational(RationalNum::from(i) + j),
                SmartNumVal::Real(j) => SmartNumVal::Real((i as f64) + j),
            },
            SmartNumVal::Rational(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Rational(i + RationalNum::from(j)),
                SmartNumVal::Rational(j) => SmartNumVal::Rational(i + j),
                SmartNumVal::Real(j) => SmartNumVal::Real(i.to_f64() + j),
            },
            SmartNumVal::Real(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Real(i + (j as f64)),
                SmartNumVal::Rational(j) => SmartNumVal::Real(i + j.to_f64()),
                SmartNumVal::Real(j) => SmartNumVal::Real(i + j),
            },
        }
    }
}

impl Sub for SmartNumVal {
    type Output = SmartNumVal;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            SmartNumVal::Integer(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Integer(i - j),
                SmartNumVal::Rational(j) => SmartNumVal::Rational(RationalNum::from(i) - j),
                SmartNumVal::Real(j) => SmartNumVal::Real((i as f64) - j),
            },
            SmartNumVal::Rational(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Rational(i - RationalNum::from(j)),
                SmartNumVal::Rational(j) => SmartNumVal::Rational(i - j),
                SmartNumVal::Real(j) => SmartNumVal::Real(i.to_f64() - j),
            },
            SmartNumVal::Real(i) => match rhs {
                SmartNumVal::Integer(j) => SmartNumVal::Real(i - (j as f64)),
                SmartNumVal::Rational(j) => SmartNumVal::Real(i - j.to_f64()),
                SmartNumVal::Real(j) => SmartNumVal::Real(i - j),
            },
        }
    }
}

impl Mul for SmartNumVal {
    type Output = SmartNumVal;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            SmartNumVal::Integer(0_i64)
        } else {
            match self {
                SmartNumVal::Integer(i) => match rhs {
                    SmartNumVal::Integer(j) => SmartNumVal::Integer(i * j),
                    SmartNumVal::Rational(j) => SmartNumVal::Rational(RationalNum::from(i) * j),
                    SmartNumVal::Real(j) => SmartNumVal::Real((i as f64) * j),
                },
                SmartNumVal::Rational(i) => match rhs {
                    SmartNumVal::Integer(j) => SmartNumVal::Rational(i * RationalNum::from(j)),
                    SmartNumVal::Rational(j) => SmartNumVal::Rational(i * j),
                    SmartNumVal::Real(j) => SmartNumVal::Real(i.to_f64() * j),
                },
                SmartNumVal::Real(i) => match rhs {
                    SmartNumVal::Integer(j) => SmartNumVal::Real(i * (j as f64)),
                    SmartNumVal::Rational(j) => SmartNumVal::Real(i * j.to_f64()),
                    SmartNumVal::Real(j) => SmartNumVal::Real(i * j),
                },
            }
        }
    }
}

impl Div for SmartNumVal {
    type Output = SmartNumVal;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            SmartNumVal::Integer(0_i64)
        } else {
            match self {
                SmartNumVal::Integer(i) => match rhs {
                    SmartNumVal::Integer(j) => {
                        SmartNumVal::Rational(RationalNum::from(i) / RationalNum::from(j))
                    }
                    SmartNumVal::Rational(j) => SmartNumVal::Rational(RationalNum::from(i) / j),
                    SmartNumVal::Real(j) => SmartNumVal::Real((i as f64) / j),
                },
                SmartNumVal::Rational(i) => match rhs {
                    SmartNumVal::Integer(j) => SmartNumVal::Rational(i / RationalNum::from(j)),
                    SmartNumVal::Rational(j) => SmartNumVal::Rational(i / j),
                    SmartNumVal::Real(j) => SmartNumVal::Real(i.to_f64() / j),
                },
                SmartNumVal::Real(i) => match rhs {
                    SmartNumVal::Integer(j) => SmartNumVal::Real(i / (j as f64)),
                    SmartNumVal::Rational(j) => SmartNumVal::Real(i / j.to_f64()),
                    SmartNumVal::Real(j) => SmartNumVal::Real(i / j),
                },
            }
        }
    }
}

impl Neg for &SmartNumVal {
    type Output = SmartNumVal;

    fn neg(self) -> Self::Output {
        match self {
            SmartNumVal::Integer(v) => SmartNumVal::Integer(-*v),
            SmartNumVal::Rational(v) => SmartNumVal::Rational(-*v),
            SmartNumVal::Real(v) => SmartNumVal::Real(-*v),
        }
    }
}

impl Neg for SmartNumVal {
    type Output = SmartNumVal;

    fn neg(self) -> Self::Output {
        -&self
    }
}
