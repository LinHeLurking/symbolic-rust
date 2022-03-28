use crate::{
    ast::ast_node::{AstNode, Expression},
    math_op::{
        add::add_eval_rule, div::div_eval_rule, mul::mul_eval_rule, neg::neg_eval_rule,
        sub::sub_eval_rule,
    },
};

pub trait NumericEvaluate {
    /// Evaluate and aggregate numbers in expression.
    /// Only arithmetic operations are evaluated.
    /// More complex operations like **sin()**, **exp()**
    /// will be left as it be.
    fn eval(self) -> Self;
    /// Evaluate as many operations as possible.
    fn full_eval(self) -> Self;
}

impl NumericEvaluate for Expression {
    fn eval(self) -> Self {
        match &self.root {
            AstNode::Operand(_) => self.clone(),
            AstNode::Operator(operator) => {
                // TODO: auto query
                if operator.descriptor == "Neg" {
                    neg_eval_rule(self.child)
                } else if operator.descriptor == "Add" {
                    add_eval_rule(self.child)
                } else if operator.descriptor == "Sub" {
                    sub_eval_rule(self.child)
                } else if operator.descriptor == "Mul" {
                    mul_eval_rule(self.child)
                } else if operator.descriptor == "Div" {
                    div_eval_rule(self.child)
                } else {
                    // Not implemented evaluation
                    self
                }
            }
        }
    }

    fn full_eval(self) -> Self {
        todo!("Not implemented yet")
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::{
        ast::ast_node::Expression,
        smart_num::{val_holder::IsClose, SmartNum, ToSmartNum},
    };

    use super::NumericEvaluate;

    #[test]
    fn eval() {
        let a = Expression::from(1_u32);
        let b = Expression::from(2_u32);
        let c = Expression::from(3_u32);
        let d = Expression::from(4_u32);
        let e = Expression::from(5_u32);
        let x = -(a + b) * (c - d) / e; // -(1+2)*(3-4)/5 == 3/5
        let ans = x.eval().to_smart_num().unwrap();
        let expected = SmartNum::new_rational(1, 3, 5).unwrap();
        assert!(ans.is_close(expected, 1e-9));
    }
}
