#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub enum SmartNum {
    Integer(i64),
    Real(f64),
}

impl Display for SmartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            SmartNum::Integer(k) => format!("{}", k),
            SmartNum::Real(f) => format!("{:.3}", f),
        };
        write!(f, "{}", content)
    }
}

impl SmartNum {
    pub fn to_i64(&self) -> i64 {
        match self {
            SmartNum::Integer(v) => *v,
            SmartNum::Real(v) => *v as i64,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            SmartNum::Real(v) => *v,
            SmartNum::Integer(v) => *v as f64,
        }
    }

    pub fn near(&self, rhs: Self, eps: f64) -> bool {
        return (self.to_f64() - rhs.to_f64()).abs() < eps;
    }

    pub fn zero() -> SmartNum {
        SmartNum::from(0_i64)
    }

    pub fn one() -> SmartNum {
        SmartNum::from(1_i64)
    }
}

impl From<i64> for SmartNum {
    fn from(v: i64) -> Self {
        SmartNum::Integer(v)
    }
}

impl From<f64> for SmartNum {
    fn from(v: f64) -> Self {
        SmartNum::Real(v)
    }
}

impl From<f32> for SmartNum {
    fn from(v: f32) -> Self {
        SmartNum::Real(v as f64)
    }
}

// Automatically generated codes for other From<T>
impl From<u8> for SmartNum {
    fn from(v: u8) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<i8> for SmartNum {
    fn from(v: i8) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<u16> for SmartNum {
    fn from(v: u16) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<i16> for SmartNum {
    fn from(v: i16) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<u32> for SmartNum {
    fn from(v: u32) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<i32> for SmartNum {
    fn from(v: i32) -> Self {
        SmartNum::Integer(i64::from(v))
    }
}

impl From<u64> for SmartNum {
    fn from(v: u64) -> Self {
        SmartNum::Integer(v as i64)
    }
}

impl Add for SmartNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            SmartNum::Integer(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Integer(i + j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) + j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i + (j as f64)),
                SmartNum::Real(j) => SmartNum::Real(i + j),
            },
        }
    }
}

impl Sub for SmartNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            SmartNum::Integer(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Integer(i - j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) - j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i - (j as f64)),
                SmartNum::Real(j) => SmartNum::Real(i - j),
            },
        }
    }
}

impl Mul for SmartNum {
    type Output = SmartNum;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            SmartNum::Integer(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Integer(i * j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) * j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i * (j as f64)),
                SmartNum::Real(j) => SmartNum::Real(i * j),
            },
        }
    }
}

impl Div for SmartNum {
    type Output = SmartNum;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            SmartNum::Integer(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real((i as f64) / (j as f64)),
                SmartNum::Real(j) => SmartNum::Real((i as f64) / (j as f64)),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real((i as f64) / (j as f64)),
                SmartNum::Real(j) => SmartNum::Real((i as f64) / (j as f64)),
            },
        }
    }
}

impl Neg for SmartNum {
    type Output = SmartNum;

    fn neg(self) -> Self::Output {
        match self {
            SmartNum::Integer(v) => SmartNum::Integer(-v),
            SmartNum::Real(v) => SmartNum::Real(-v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmartNum;

    fn gen_range() -> Vec<i64> {
        let mut rg = Vec::<i64>::new();
        for i in -1000_i64..1000_i64 {
            rg.push(i);
        }
        return rg;
    }

    fn gen_double_range() -> Vec<(i64, i64)> {
        let mut rg = Vec::<(i64, i64)>::new();
        for i in gen_range() {
            for j in gen_range() {
                rg.push((i, j));
            }
        }
        return rg;
    }

    fn is_close(a: f64, b: f64) -> bool {
        return (a - b).abs() < 1e-9;
    }

    #[test]
    fn neg_i() {
        for i in gen_range() {
            let x = SmartNum::from(i);
            let y = SmartNum::from(-i);
            assert_eq!(x.to_i64(), (-y).to_i64())
        }
    }

    #[test]
    fn neg_f() {
        for i in gen_range() {
            let x = SmartNum::from(i as f64);
            let y = SmartNum::from(-i as f64);
            assert!(is_close(x.to_f64(), (-y).to_f64()));
        }
    }

    #[test]
    fn add_ii() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x + y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    #[test]
    fn add_ff() {
        for (i, j) in gen_double_range() {
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x + y);
            assert!(is_close(ans.to_f64(), check.to_f64()));
        }
    }

    #[test]
    fn add_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x as f64 + y);
            assert!(is_close(ans.to_f64(), check.to_f64()));
        }
    }

    #[test]
    fn mul_ii() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x * y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    #[test]
    fn mul_ff() {
        for (i, j) in gen_double_range() {
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x * y);
            assert!(is_close(ans.to_f64(), check.to_f64()));
        }
    }

    #[test]
    fn mul_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x as f64 * y);
            assert!(is_close(ans.to_f64(), check.to_f64()));
        }
    }

    #[test]
    fn string_fmt() {
        for i in gen_range() {
            let x = i.clone() as i64;
            let y = i.clone() as f64;
            let a = SmartNum::from(x);
            let b = SmartNum::from(y);
            assert_eq!(a.to_string(), format!("{}", x));
            assert_eq!(b.to_string(), format!("{:.3}", y));
        }
    }
}
