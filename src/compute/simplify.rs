use crate::{
    ast::ast_node::{AstNode, Expression},
    math_op::neg::OP_NEG,
};

trait NumAggregateSimplify {
    fn num_aggregate(&self) -> Self;
}

pub trait Simplify {
    fn simplify(&self) -> Self;
}

impl<'a> NumAggregateSimplify for Expression<'a> {
    fn num_aggregate(&self) -> Self {
        match &self.root {
            AstNode::Operand(_) => self.clone(),
            AstNode::Operator(operator) => {
                if operator.descriptor == OP_NEG.descriptor {
                    let sub = self.child[0].num_aggregate();
                    if sub.is_num() {
                        Expression::from(-sub.to_smart_num().unwrap())
                    } else {
                        -sub
                    }
                } else {
                    panic!("Aggregation not Implemented for {}", operator.symbol);
                }
            }
        }
    }
}

impl<'a> Simplify for Expression<'a> {
    fn simplify(&self) -> Self {
        self.num_aggregate()
    }
}

// #[test]
// fn num_aggregate() {
//     {
//         let a = Expression::from(1_f32);
//         let b = Expression::from(2_u32);
//         let c = Expression::from(3_i64);
//         let d = Expression::from(4_f64);
//         let e = Expression::from(3_i32);
//         let mut x = (a + b) * (c - d) / e; // (1.0f+2)*(3-4.0)/3 == -1.0
//         dbg!(x.to_string());
//         x.simplify();
//         dbg!(x.to_string());
//         assert!(x.is_num());
//         assert!(x.to_smart_num().near(SmartNum::from(-1.0), 1e-9));
//     }
// }
