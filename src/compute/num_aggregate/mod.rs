mod rules;

use crate::ast::{
    op::operator::OperatorType,
    tree::{AstNode, Expression},
};

use self::rules::{
    add::add_eval_rule, cos::cos_eval_rule, div::div_eval_rule, exp::exp_eval_rule,
    ln::ln_eval_rule, mul::mul_eval_rule, neg::neg_eval_rule, pow::pow_eval_rule,
    sin::sin_eval_rule, sub::sub_eval_rule,
};

pub trait NumAggregate {
    /// Evaluate and aggregate numbers in expression.
    /// Only arithmetic operations are evaluated.
    /// More complex operations like **sin()**, **exp()**
    /// will be left as it be.
    fn num_aggregate(self) -> Self;
}

impl NumAggregate for Expression {
    fn num_aggregate(self) -> Self {
        match &self.root {
            AstNode::Operand(_) => self,
            AstNode::Operator(operator) => match operator.descriptor {
                OperatorType::Neg => neg_eval_rule(self.child),
                OperatorType::Add => add_eval_rule(self.child),
                OperatorType::Sub => sub_eval_rule(self.child),
                OperatorType::Mul => mul_eval_rule(self.child),
                OperatorType::Div => div_eval_rule(self.child),
                OperatorType::Sin => sin_eval_rule(self.child),
                OperatorType::Cos => cos_eval_rule(self.child),
                OperatorType::Exp => exp_eval_rule(self.child),
                OperatorType::Ln => ln_eval_rule(self.child),
                OperatorType::Pow => pow_eval_rule(self.child),
            },
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::{
        ast::tree::Expression,
        smart_num::{val_holder::IsClose, SmartNum, ToSmartNum},
    };

    use super::NumAggregate;

    #[test]
    fn eval() {
        let a = Expression::from(1_u32);
        let b = Expression::from(2_u32);
        let c = Expression::from(3_u32);
        let d = Expression::from(4_u32);
        let e = Expression::from(5_u32);
        let x = -(a + b) * (c - d) / e; // -(1+2)*(3-4)/5 == 3/5
        let ans = x.num_aggregate().to_smart_num().unwrap();
        let expected = SmartNum::new_rational(1, 3, 5).unwrap();
        assert!(ans.is_close(expected, 1e-9));
    }
}
