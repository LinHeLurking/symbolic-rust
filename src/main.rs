use crate::compute::taylor_expansion::partial_expansion::*;
use crate::math_op::ln::ln;
use crate::{ast::tree::Expression, math_op::exp::exp};

mod ast;
mod compute;
mod math_op;
mod smart_num;

fn main() {
    println!("Hello, world!");
    {
        let zero = Expression::zero();
        let x = Expression::new_variable("x");
        let exp_x = exp(x.clone());
        println!(
            "Taylor expansion of {}: {}",
            exp_x.to_string(),
            exp_x.taylor_expansion(x, zero, 5).unwrap()
        );
    }
    {
        let one = Expression::one();
        let x = Expression::new_variable("x");
        let ln_x = ln(x.clone());
        println!(
            "Taylor expansion of {}: {}",
            ln_x.to_string(),
            ln_x.taylor_expansion(x, one, 5).unwrap()
        );
    }
}
