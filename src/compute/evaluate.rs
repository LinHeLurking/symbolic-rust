use crate::{
    ast::ast_node::{AstNode, Expression},
    math_op::{add::OP_ADD, div::OP_DIV, mul::OP_MUL, neg::OP_NEG, sub::OP_SUB},
};

pub trait NumericEvaluate {
    /// Evaluate and aggregate numbers in expression.
    /// Only arithmetic operations are evaluated.
    /// More complex operations like **sin()**, **exp()**
    /// will be left as it be.
    fn eval(&self) -> Self;
    /// Evaluate as many operations as possible.
    fn full_eval(&self) -> Self;
}

impl<'a> NumericEvaluate for Expression<'a> {
    fn eval(&self) -> Self {
        match &self.root {
            AstNode::Operand(_) => self.clone(),
            AstNode::Operator(operator) => {
                if operator.descriptor == OP_NEG.descriptor {
                    let sub = self.child[0].eval();
                    if sub.is_num() {
                        Expression::from(-sub.to_smart_num().unwrap())
                    } else {
                        -sub
                    }
                } else if operator.descriptor == OP_ADD.descriptor {
                    let l = self.child[0].eval();
                    let r = self.child[1].eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() + r.to_smart_num().unwrap())
                    } else {
                        l + r
                    }
                } else if operator.descriptor == OP_SUB.descriptor {
                    let l = self.child[0].eval();
                    let r = self.child[1].eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() - r.to_smart_num().unwrap())
                    } else {
                        l - r
                    }
                } else if operator.descriptor == OP_MUL.descriptor {
                    let l = self.child[0].eval();
                    let r = self.child[1].eval();
                    if l.is_num() && r.is_num() {
                        Expression::from(l.to_smart_num().unwrap() * r.to_smart_num().unwrap())
                    } else {
                        l * r
                    }
                } else if operator.descriptor == OP_DIV.descriptor {
                    let l = self.child[0].eval();
                    let r = self.child[1].eval();
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

    fn full_eval(&self) -> Self {
        let x = self.eval();
        return x;
    }
}
