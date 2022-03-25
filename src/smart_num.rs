#![allow(dead_code)]

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub enum SmartNum {
    Integer(i64),
    Real(f64),
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

impl SmartNum {
    fn to_i64(&self) -> i64 {
        match self {
            SmartNum::Integer(k) => k.clone(),
            _ => panic!("Unreachable"),
        }
    }

    fn to_f64(&self) -> f64 {
        match self {
            SmartNum::Real(f) => f.clone(),
            _ => panic!("Unreachable"),
        }
    }
}

fn is_close(a: f64, b: f64) -> bool {
    return (a - b).abs() < 1e-9;
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

fn gen_range() -> Vec<(i64, i64)> {
    let mut rg = Vec::<(i64, i64)>::new();
    for i in -1000_i64..1000_i64 {
        for j in -1000_i64..1000_i64 {
            rg.push((i, j));
        }
    }
    return rg;
}

#[test]
fn add_ii() {
    for (i, j) in gen_range() {
        let x = i as i64;
        let y = j as i64;
        let ans = SmartNum::from(x) + SmartNum::from(y);
        let check = SmartNum::from(x + y);
        assert_eq!(ans.to_i64(), check.to_i64());
    }
}

#[test]
fn add_ff() {
    for (i, j) in gen_range() {
        let x = i as f64;
        let y = j as f64;
        let ans = SmartNum::from(x) + SmartNum::from(y);
        let check = SmartNum::from(x + y);
        assert!(is_close(ans.to_f64(), check.to_f64()));
    }
}

#[test]
fn add_if() {
    for (i, j) in gen_range() {
        let x = i as i64;
        let y = j as f64;
        let ans = SmartNum::from(x) + SmartNum::from(y);
        let check = SmartNum::from(x as f64 + y);
        assert!(is_close(ans.to_f64(), check.to_f64()));
    }
}

#[test]
fn mul_ii() {
    for (i, j) in gen_range() {
        let x = i as i64;
        let y = j as i64;
        let ans = SmartNum::from(x) * SmartNum::from(y);
        let check = SmartNum::from(x * y);
        assert_eq!(ans.to_i64(), check.to_i64());
    }
}

#[test]
fn mul_ff() {
    for (i, j) in gen_range() {
        let x = i as f64;
        let y = j as f64;
        let ans = SmartNum::from(x) * SmartNum::from(y);
        let check = SmartNum::from(x * y);
        assert!(is_close(ans.to_f64(), check.to_f64()));
    }
}

#[test]
fn mul_if() {
    for (i, j) in gen_range() {
        let x = i as i64;
        let y = j as f64;
        let ans = SmartNum::from(x) * SmartNum::from(y);
        let check = SmartNum::from(x as f64 * y);
        assert!(is_close(ans.to_f64(), check.to_f64()));
    }
}
