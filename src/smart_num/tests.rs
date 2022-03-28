#[cfg(test)]
mod smart_num_tests {
    use crate::smart_num::val_holder::IsClose;

    use super::super::SmartNum;

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
            assert!(x.is_close(-y, 1e-9));
        }
    }

    fn neg_f() {
        for i in gen_range() {
            let x = SmartNum::from(i as f64);
            let y = SmartNum::from(-i as f64);
            assert!(x.is_close(-y, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
        }
    }

    fn add_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) + SmartNum::from(y);
            let check = SmartNum::from(x as f64 + y);
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
        }
    }

    fn sub_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) - SmartNum::from(y);
            let check = SmartNum::from(x as f64 - y);
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
        }
    }

    fn mul_if() {
        for (i, j) in gen_double_range() {
            let x = i as i64;
            let y = j as f64;
            let ans = SmartNum::from(x) * SmartNum::from(y);
            let check = SmartNum::from(x as f64 * y);
            assert!(ans.is_close(check, 1e-9));
        }
    }

    fn mul_ir() {
        let i = -100_i64;
        let j = -99_i64;
        let k = 99_i64;
        let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
        let check = SmartNum::from((i as f64) * (j as f64) / (k as f64));
        let ans =
            SmartNum::from(i) * SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
        assert!(ans.is_close(check, 1e-9));
        for (i, j, k) in gen_triple_range() {
            if k == 0 {
                continue;
            }
            let s = if j * k >= 0 { 1_i64 } else { -1_i64 };
            let check = SmartNum::from((i as f64) * (j as f64) / (k as f64));
            let ans = SmartNum::from(i)
                * SmartNum::new_rational(s, j.abs() as u64, k.abs() as u64).unwrap();
            assert!(ans.is_close(check, 1e-9));
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
            let ans = SmartNum::from(i) / SmartNum::from(j);
            let s = if i == 0 { 1 } else { (i * j).signum() };
            let check = SmartNum::new_rational(s, i.abs() as u64, j.abs() as u64).unwrap();
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
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
            assert!(ans.is_close(check, 1e-9));
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

#[cfg(test)]
mod rational_tests {
    use super::super::rational::RationalNum;

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

#[cfg(test)]
mod special_const_tests {
    use crate::smart_num::{val_holder::IsClose, SmartNum};

    #[test]
    fn string_fmt() {
        let pi = SmartNum::pi();
        assert_eq!(pi.to_string(), "pi");
        assert!(pi.is_close(SmartNum::from(3.14), 1e-1));
        let e = SmartNum::e();
        assert_eq!(e.to_string(), "e");
        assert!(e.is_close(SmartNum::from(2.7), 1e-1));
    }
}
