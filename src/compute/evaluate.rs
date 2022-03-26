use crate::ast::ast_node::{AstNode, Expression};

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
        match self.root {
            AstNode::Operand(_) => self.clone(),
            AstNode::Operator(operator) => {
                let mut child = self.child;
                if operator.descriptor == "Neg" {
                    let sub = child.pop().unwrap().eval();
                    if sub.is_num() {
                        Expression::from(-sub.to_smart_num().unwrap())
                    } else {
                        -sub
                    }
                } else if operator.descriptor == "Add" {
                    let l = child.pop().unwrap().eval();
                    let r = child.pop().unwrap().eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() + r.to_smart_num().unwrap())
                    } else {
                        l + r
                    }
                } else if operator.descriptor == "Sub" {
                    let l = child.pop().unwrap().eval();
                    let r = child.pop().unwrap().eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() - r.to_smart_num().unwrap())
                    } else {
                        l - r
                    }
                } else if operator.descriptor == "Mul" {
                    let l = child.pop().unwrap().eval();
                    let r = child.pop().unwrap().eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() * r.to_smart_num().unwrap())
                    } else {
                        l * r
                    }
                } else if operator.descriptor == "Div" {
                    let l = child.pop().unwrap().eval();
                    let r = child.pop().unwrap().eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() / r.to_smart_num().unwrap())
                    } else {
                        l / r
                    }
                } else {
                    panic!("Aggregation not Implemented for {}", operator.symbol);
                }
            }
        }
    }

    fn full_eval(self) -> Self {
        let x = self.eval();
        return x;
    }
}
