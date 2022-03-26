#![allow(dead_code)]

mod rational;

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    panic,
};

use self::rational::RationalNum;

#[derive(Debug, Clone, Copy)]
pub enum SmartNum {
    Integer(i64),
    Rational(RationalNum),
    Real(f64),
}

impl Display for SmartNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            SmartNum::Integer(v) => format!("{}", v),
            SmartNum::Rational(v) => format!("{}", v),
            SmartNum::Real(v) => format!("{:.3}", v),
        };
        write!(f, "{}", content)
    }
}

impl SmartNum {
    pub fn to_i64(&self) -> i64 {
        match self {
            SmartNum::Integer(v) => *v,
            SmartNum::Rational(_) => panic!("Cannot convert a rational number into i64"),
            SmartNum::Real(v) => *v as i64,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            SmartNum::Real(v) => *v,
            SmartNum::Rational(v) => v.to_f64(),
            SmartNum::Integer(v) => *v as f64,
        }
    }

    pub fn new_rational(sign: i64, nominator: u64, denominator: u64) -> Option<SmartNum> {
        RationalNum::new(sign, nominator, denominator).and_then(|v| Some(SmartNum::Rational(v)))
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
                SmartNum::Rational(j) => SmartNum::Rational(RationalNum::from(i) + j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) + j),
            },
            SmartNum::Rational(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Rational(i + RationalNum::from(j)),
                SmartNum::Rational(j) => SmartNum::Rational(i + j),
                SmartNum::Real(j) => SmartNum::Real(i.to_f64() + j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i + (j as f64)),
                SmartNum::Rational(j) => SmartNum::Real(i + j.to_f64()),
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
                SmartNum::Rational(j) => SmartNum::Rational(RationalNum::from(i) - j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) - j),
            },
            SmartNum::Rational(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Rational(i - RationalNum::from(j)),
                SmartNum::Rational(j) => SmartNum::Rational(i - j),
                SmartNum::Real(j) => SmartNum::Real(i.to_f64() - j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i - (j as f64)),
                SmartNum::Rational(j) => SmartNum::Real(i - j.to_f64()),
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
                SmartNum::Rational(j) => SmartNum::Rational(RationalNum::from(i) * j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) * j),
            },
            SmartNum::Rational(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Rational(i * RationalNum::from(j)),
                SmartNum::Rational(j) => SmartNum::Rational(i * j),
                SmartNum::Real(j) => SmartNum::Real(i.to_f64() * j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i * (j as f64)),
                SmartNum::Rational(j) => SmartNum::Real(i * j.to_f64()),
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
                SmartNum::Integer(j) => SmartNum::Integer(i / j),
                SmartNum::Rational(j) => SmartNum::Rational(RationalNum::from(i) / j),
                SmartNum::Real(j) => SmartNum::Real((i as f64) / j),
            },
            SmartNum::Rational(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Rational(i / RationalNum::from(j)),
                SmartNum::Rational(j) => SmartNum::Rational(i / j),
                SmartNum::Real(j) => SmartNum::Real(i.to_f64() / j),
            },
            SmartNum::Real(i) => match rhs {
                SmartNum::Integer(j) => SmartNum::Real(i / (j as f64)),
                SmartNum::Rational(j) => SmartNum::Real(i / j.to_f64()),
                SmartNum::Real(j) => SmartNum::Real(i / j),
            },
        }
    }
}

impl Neg for SmartNum {
    type Output = SmartNum;

    fn neg(self) -> Self::Output {
        match self {
            SmartNum::Integer(v) => SmartNum::Integer(-v),
            SmartNum::Rational(v) => SmartNum::Rational(-v),
            SmartNum::Real(v) => SmartNum::Real(-v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmartNum;

    fn gen_range() -> Vec<i64> {
        let mut rg = Vec::<i64>::new();
        for i in -100_i64..100_i64 {
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

    fn gen_triple_range() -> Vec<(i64, i64, i64)> {
        let mut rg = Vec::<(i64, i64, i64)>::new();
        for i in gen_range() {
            for j in gen_range() {
                for k in gen_range() {
                    rg.push((i, j, k));
                }
            }
        }
        return rg;
    }

    fn neg_i() {
        for i in gen_range() {
            let x = SmartNum::from(i);
            let y = SmartNum::from(-i);
            assert!(x.near(-y, 1e-9));
        }
    }

    fn neg_f() {
        for i in gen_range() {
            let x = SmartNum::from(i as f64);
            let y = SmartNum::from(-i as f64);
            assert!(x.near(-y, 1e-9));
        }
    }

    #[test]
    fn neg() {
        neg_i();
        neg_f();
    }

    fn add_ii() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x + y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    fn add_ff() {
        for (i, j) in gen_double_range() {
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x + y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn add_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x as f64 + y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn add_ir() {
        for (i, j, k) in gen_triple_range() {
            if k == 0 {
                continue;
            }
            let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
            let check = SmartNum::from((i as f64) + (j as f64) / (k as f64));
            let ans = SmartNum::from(i)
                + SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
            assert!(ans.near(check, 1e-9));
        }
    }

    #[test]
    fn add() {
        add_ii();
        add_ff();
        add_if();
        add_ir();
    }

    fn sub_ii() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) - SmartNum::from(y);
            let check = SmartNum::from(x - y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    fn sub_ff() {
        for (i, j) in gen_double_range() {
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) - SmartNum::from(y);
            let check = SmartNum::from(x - y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn sub_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) - SmartNum::from(y);
            let check = SmartNum::from(x as f64 - y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn sub_ir() {
        for (i, j, k) in gen_triple_range() {
            if k == 0 {
                continue;
            }
            let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
            let check = SmartNum::from((i as f64) - (j as f64) / (k as f64));
            let ans = SmartNum::from(i)
                - SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
            assert!(ans.near(check, 1e-9));
        }
    }

    #[test]
    fn sub() {
        sub_ii();
        sub_ff();
        sub_ff();
        sub_ir();
    }

    fn mul_ii() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x * y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    fn mul_ff() {
        for (i, j) in gen_double_range() {
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x * y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn mul_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x as f64 * y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn mul_ir() {
        for (i, j, k) in gen_triple_range() {
            if k == 0 {
                continue;
            }
            let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
            let check = SmartNum::from((i as f64) * (j as f64) / (k as f64));
            let ans = SmartNum::from(i)
                * SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
            assert!(ans.near(check, 1e-9));
        }
    }

    #[test]
    fn mul() {
        mul_ii();
        mul_ff();
        mul_if();
        mul_ir();
    }

    fn div_ii() {
        for (i, j) in gen_double_range() {
            if j == 0 {
                continue;
            }
            let x = i as i64;
            let y = j as i64;
            let ans = SmartNum::from(x) / SmartNum::from(y);
            let check = SmartNum::from(x / y);
            assert_eq!(ans.to_i64(), check.to_i64());
        }
    }

    fn div_ff() {
        for (i, j) in gen_double_range() {
            if j == 0 {
                continue;
            }
            let x = i as f64;
            let y = j as f64;
            let ans = SmartNum::from(x) / SmartNum::from(y);
            let check = SmartNum::from(x / y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn div_if() {
        for (i, j) in gen_double_range() {
            if j == 0 {
                continue;
            }
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) / SmartNum::from(y);
            let check = SmartNum::from(x as f64 / y);
            assert!(ans.near(check, 1e-9));
        }
    }

    fn div_ir() {
        for (i, j, k) in gen_triple_range() {
            if k == 0 || j == 0 {
                continue;
            }
            let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
            let check = SmartNum::from((i as f64) / ((j as f64) / (k as f64)));
            let ans = SmartNum::from(i)
                / SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
            assert!(ans.near(check, 1e-9));
        }
    }

    #[test]
    fn div() {
        div_ii();
        div_ff();
        div_if();
        div_ir();
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
