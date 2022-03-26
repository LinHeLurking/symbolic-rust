use std::ops::Neg;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

pub(crate) const OP_NEG: AstOperator = AstOperator {
    symbol: "-",
    priority: 1_u32,
    descriptor: "Neg",
};

impl<'a> Neg for Expression<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expression {
            root: AstNode::Operator(OP_NEG.clone()),
            child: vec![self],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Expression;
    use crate::compute::evaluate::NumericEvaluate;

    #[test]
    fn neg() {
        let x = Expression::from(1_i32);
        let y = Expression::from(-1_i32);
        let z = (-y).eval();
        assert_eq!(
            x.to_smart_num().unwrap().to_i64(),
            z.to_smart_num().unwrap().to_i64()
        );
    }
}
