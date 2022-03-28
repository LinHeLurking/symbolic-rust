// use std::{error::Error, fmt::Display};

// use crate::ast::{
//     op::operand::{AstOperand, Variable},
//     tree::{AstNode, Expression},
// };

// mod rules;

// pub(crate) trait Substitute<T, U> {
//     type Output;
//     /// substitute every **u** with **v**.
//     fn substitute(self, u: T, v: U) -> Self::Output;
// }

// #[derive(Debug)]
// pub(crate) struct SubstituteError {}

// impl Display for SubstituteError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Cannot perform substitution")
//     }
// }

// impl<'a, T, U> Substitute<T, U> for Expression
// where
//     T: Into<Option<&'a Variable>>,
//     U: Into<Expression> + Clone,
// {
//     type Output = Result<Expression, SubstituteError>;

//     fn substitute(self, u: T, v: U) -> Self::Output {
//         let src = Into::<Option<&'a Variable>>::into(&u).ok_or(SubstituteError {})?;
//         let result = match &self.root {
//             AstNode::Operand(operand) => match operand {
//                 AstOperand::Num(_) => self,
//                 AstOperand::Variable(variable) => {
//                     if variable.name == src.name {
//                         v.clone().into()
//                     } else {
//                         self
//                     }
//                 }
//             },
//             AstNode::Operator(_) => {
//                 let mut s = self;
//                 let mut c: Vec<Expression> = vec![];
//                 for expr in s.child {
//                     c.push(expr.substitute(src, v.clone())?);
//                 }
//                 s.child = c;
//                 s
//             }
//         };
//         return Ok(result);
//     }
// }

// #[cfg(test)]
// mod substitute_tests {
//     use crate::{ast::tree::Expression, compute::num_aggregate::NumAggregate};

//     use super::Substitute;

//     #[test]
//     fn substitute() {
//         let a = Expression::new_variable("a");
//         let b = Expression::new_variable("b");
//         let target = Expression::from(1_i64);
//         let y = a.clone() * a.clone() + b.clone() * b.clone();
//         let y_s = y.substitute(&a, target).unwrap().num_aggregate();
//         println!("{}", y_s);
//         assert_eq!(y_s.to_string(), "1 + b * b");
//     }
// }
