// #![allow(dead_code)]

// use crate::{
//     ast::{op::operand::Variable, tree::Expression},
//     compute::derivative::Derivative,
//     smart_num::SmartNum,
// };

// pub struct PartialExpansion {
//     order: u32,
//     coefficient: Vec<SmartNum>,
//     residual: Expression,
// }

// trait TaylorExpansion<U, T>
// where
//     U: Into<Option<Variable>>,
//     T: Into<Expression>,
// {
//     type Output;
//     fn taylor_expansion(self, of: U, at: &T) -> Self::Output;
// }

// impl<'a, T> Derivative<T> for &'a mut PartialExpansion
// where
//     T: Into<Option<Variable>>,
// {
//     type Output = &'a mut PartialExpansion;

//     fn derivative(self, to: T) -> Self::Output {
//         let to_ = Into::<Option<Variable>>::into(to).unwrap();
//         self.order+=1;
//         // self.coefficient.push(self.)
//     }
// }
