use std::ops::Add;

use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
};

pub(crate) const OP_ADD: AstOperator = AstOperator {
    symbol: "+",
    priority: 2_u32,
    descriptor: "Add",
};

impl<'a> Add for Expression<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Expression {
            root: AstNode::Operator(OP_ADD.clone()),
            child: vec![self, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast_node::Expression, compute::simplify::Simplify};

    #[test]
    fn add() {
        {
            let x = Expression::new_variable("x");
            let y = Expression::new_variable("y");
            let z = x + y;
            assert_eq!(z.to_string(), "x + y");
        }
        {
            let x = Expression::from(1_u32);
            let y = Expression::from(1_u32);
            let z = (x + y).simplify();
            let expected = Expression::from(2_u32);
            assert!(z.near(&expected, 1e-9).unwrap());
        }
    }
}
