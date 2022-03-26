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

impl<'a> AstNode<'a> {
    pub fn gen_neg_op_node() -> AstNode<'a> {
        AstNode::Operator(OP_NEG.clone())
    }
}

#[cfg(test)]
mod test {
    use super::Expression;
    use crate::compute::simplify::Simplify;

    #[test]
    fn neg() {
        let x = Expression::from(1_i32);
        let y = Expression::from(-1_i32);
        let z = (-y).simplify();
        assert_eq!(
            x.to_smart_num().unwrap().to_i64(),
            z.to_smart_num().unwrap().to_i64()
        );
    }
}
