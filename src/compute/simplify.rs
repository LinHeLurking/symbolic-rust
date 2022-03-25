use crate::ast::{
    ast_node::{AstNode, Expression},
    op::AstOperator,
    smart_num::SmartNum,
};

trait NumAggregateSimplify {
    fn num_aggregate(&mut self) -> &Self;
}

pub trait CanSimplify {
    fn simplify(&mut self) -> &Self;
}

impl NumAggregateSimplify for Expression {
    fn num_aggregate(&mut self) -> &Expression {
        let expr = match &self.me {
            AstNode::Operand(_) => self,
            AstNode::Operator(op) => {
                let mut flag = true;
                for expr in self.child.iter_mut() {
                    expr.num_aggregate();
                    flag &= expr.is_num();
                }
                if flag {
                    match op {
                        AstOperator::Neg(_) => {
                            let a = self.child[0].to_smart_num();
                            self.child.clear();
                            self.me = AstNode::from(a);
                        }
                        AstOperator::Add(_) => {
                            let a = self.child[0].to_smart_num();
                            let b = self.child[1].to_smart_num();
                            self.child.clear();
                            self.me = AstNode::from(a + b);
                        }
                        AstOperator::Sub(_) => {
                            let a = self.child[0].to_smart_num();
                            let b = self.child[1].to_smart_num();
                            self.child.clear();
                            self.me = AstNode::from(a - b);
                        }
                        AstOperator::Mul(_) => {
                            let a = self.child[0].to_smart_num();
                            let b = self.child[1].to_smart_num();
                            self.child.clear();
                            self.me = AstNode::from(a * b);
                        }
                        AstOperator::Div(_) => {
                            let a = self.child[0].to_smart_num();
                            let b = self.child[1].to_smart_num();
                            self.child.clear();
                            self.me = AstNode::from(a / b);
                        }
                    }
                }
                self
            }
        };
        return expr;
    }
}

impl CanSimplify for Expression {
    fn simplify(&mut self) -> &Self {
        self.num_aggregate();
        return self;
    }
}

#[test]
fn num_aggregate() {
    {
        let a = Expression::from(1_f32);
        let b = Expression::from(2_u32);
        let c = Expression::from(3_i64);
        let d = Expression::from(4_f64);
        let e = Expression::from(3_i32);
        let mut x = (a + b) * (c - d) / e; // (1.0f+2)*(3-4.0)/3 == -1.0
        dbg!(x.to_string());
        x.simplify();
        dbg!(x.to_string());
        assert!(x.is_num());
        assert!(x.to_smart_num().near(SmartNum::from(-1.0), 1e-9));
    }
}
