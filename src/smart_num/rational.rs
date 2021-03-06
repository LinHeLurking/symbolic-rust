use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct RationalNum {
    pub sign: i64,
    pub nominator: u64,
    pub denominator: u64,
}

pub trait ToRational {
    type Output;
    fn to_rational(self) -> Self::Output;
}

impl Display for RationalNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_symbol = if self.sign == 1 { "" } else { "-" };
        write!(f, "{}{}/{}", sign_symbol, self.nominator, self.denominator)
    }
}

impl PartialEq for RationalNum {
    fn eq(&self, other: &Self) -> bool {
        let x = self.reduce();
        let y = other.reduce();
        return (x.sign == y.sign && x.nominator == y.nominator && x.denominator == y.denominator)
            || (x.nominator == 0 && y.nominator == 0);
    }
}

impl Eq for RationalNum {}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl RationalNum {
    pub fn reduce(&self) -> Self {
        let mut d = gcd(self.nominator, self.denominator);
        if d == 0 {
            d = 1
        }
        RationalNum {
            sign: self.sign,
            nominator: self.nominator / d,
            denominator: self.denominator / d,
        }
    }

    pub fn new(sign: i64, nominator: u64, denominator: u64) -> Option<RationalNum> {
        if sign.abs() == 1 && denominator != 0 {
            Some(RationalNum {
                sign,
                nominator,
                denominator,
            })
        } else {
            None
        }
    }

    pub fn new_positive(nominator: u64, denominator: u64) -> Option<RationalNum> {
        return RationalNum::new(1_i64, nominator, denominator);
    }

    pub fn new_negative(nominator: u64, denominator: u64) -> Option<RationalNum> {
        return RationalNum::new(-1_i64, nominator, denominator);
    }

    pub fn to_f64(&self) -> f64 {
        (self.sign as f64) * (self.nominator as f64) / (self.denominator as f64)
    }
}

impl Neg for RationalNum {
    type Output = RationalNum;

    fn neg(self) -> Self::Output {
        return RationalNum {
            sign: -self.sign,
            ..self
        };
    }
}

impl Add for RationalNum {
    type Output = RationalNum;

    fn add(self, rhs: Self) -> Self::Output {
        let a = ((self.nominator * rhs.denominator) as i64) * self.sign;
        let b = ((self.denominator * rhs.nominator) as i64) * rhs.sign;
        let s = a + b;
        let sign = s.signum();
        let nominator = s.abs() as u64;
        let denominator = self.denominator * rhs.denominator;
        let res = RationalNum {
            sign,
            nominator,
            denominator,
        };
        res.reduce()
    }
}

impl Sub for RationalNum {
    type Output = RationalNum;

    fn sub(self, rhs: Self) -> Self::Output {
        let a = ((self.nominator * rhs.denominator) as i64) * self.sign;
        let b = ((self.denominator * rhs.nominator) as i64) * rhs.sign;
        let s = a - b;
        let sign = s.signum();
        let nominator = s.abs() as u64;
        let denominator = self.denominator * rhs.denominator;
        let res = RationalNum {
            sign,
            nominator,
            denominator,
        };
        res.reduce()
    }
}

impl Mul for RationalNum {
    type Output = RationalNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign * rhs.sign;
        let nominator = self.nominator * rhs.nominator;
        let denominator = self.denominator * rhs.denominator;
        let res = RationalNum {
            sign,
            nominator,
            denominator,
        };
        res.reduce()
    }
}

impl Div for RationalNum {
    type Output = RationalNum;

    fn div(self, rhs: Self) -> Self::Output {
        let sign = self.sign * rhs.sign;
        let nominator = self.nominator * rhs.denominator;
        let denominator = self.denominator * rhs.nominator;
        let res = RationalNum {
            sign,
            nominator,
            denominator,
        };
        res.reduce()
    }
}

impl From<u64> for RationalNum {
    fn from(v: u64) -> Self {
        RationalNum {
            sign: 1,
            nominator: v,
            denominator: 1,
        }
    }
}

impl From<i64> for RationalNum {
    fn from(v: i64) -> Self {
        RationalNum {
            sign: if v == 0 { 1_i64 } else { v.signum() },
            nominator: v.abs() as u64,
            denominator: 1,
        }
    }
}
