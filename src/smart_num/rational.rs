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

#[cfg(test)]
mod tests {
    use super::RationalNum;

    #[test]
    fn eq() {
        {
            let x = RationalNum::new_positive(0_u64, 3_u64).unwrap();
            let y = RationalNum::new_negative(0_u64, 5_u64).unwrap();
            assert_eq!(x, y);
        }
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            assert_eq!(x, y);
        }
        {
            let x = RationalNum::new_positive(3_u64, 9_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 3_u64).unwrap();
            assert_eq!(x, y);
        }
    }

    #[test]
    fn neg() {
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            assert_eq!(x, -y);
        }
        {
            let x = RationalNum::new_positive(1_u64, 3_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 3_u64).unwrap();
            assert_eq!(x, -y);
        }
    }

    #[test]
    fn add() {
        {
            let x = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            assert_eq!(x + y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            assert_eq!(x + y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let z = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            assert_eq!(x + y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            assert_eq!(x + y, z);
        }
    }

    #[test]
    fn sub() {
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            assert_eq!(x - y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_negative(5_u64, 6_u64).unwrap();
            assert_eq!(x - y, z);
        }
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_positive(5_u64, 6_u64).unwrap();
            assert_eq!(x - y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            assert_eq!(x - y, z);
        }
    }

    #[test]
    fn mul() {
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(1_u64, 8_u64).unwrap();
            assert_eq!(x * y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_negative(1_u64, 6_u64).unwrap();
            assert_eq!(x * y, z);
        }
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_negative(1_u64, 6_u64).unwrap();
            assert_eq!(x * y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(1_u64, 8_u64).unwrap();
            assert_eq!(x * y, z);
        }
    }

    #[test]
    fn div() {
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(2_u64, 1_u64).unwrap();
            assert_eq!(x / y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_positive(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_negative(3_u64, 2_u64).unwrap();
            assert_eq!(x / y, z);
        }
        {
            let x = RationalNum::new_positive(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 3_u64).unwrap();
            let z = RationalNum::new_negative(3_u64, 2_u64).unwrap();
            assert_eq!(x / y, z);
        }
        {
            let x = RationalNum::new_negative(1_u64, 2_u64).unwrap();
            let y = RationalNum::new_negative(1_u64, 4_u64).unwrap();
            let z = RationalNum::new_positive(2_u64, 1_u64).unwrap();
            assert_eq!(x / y, z);
        }
    }
}
