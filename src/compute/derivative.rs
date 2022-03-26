use crate::ast::op::Variable;

pub trait CanDerivative {
    type Output;
    fn derivative(&self, x: &Variable) -> Self::Output;
}

// #[test]
// fn derivative_constant() {
//     {
//         let a = Expression::from(1_f32);
//         let b = Expression::from(2_u32);
//         let c = Expression::from(3_i64);
//         let d = Expression::from(4_f64);
//         let x = (a + b) * (c - d);
//         let y = x.clone().derivative(&Variable::new_variable("x"));
//         assert!(y.is_num());
//         assert!(y.to_smart_num().near(SmartNum::from(0_i64), 1e-9));
//     }
// }

// #[test]
// fn derivative_variable() {
//     {
//         let x = Variable::new_variable("x");
//         let f = Expression::new_variable("x");
//         let y = f.derivative(&x);
//         assert!(y.is_num());
//         assert!(y.to_smart_num().near(SmartNum::one(), 1e-9));
//     }
// }
