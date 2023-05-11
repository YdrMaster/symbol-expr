mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use ast::Algebra;

fn main() {
    let algebra = "1+2*3/4%(a_+B*_c/d0)".parse::<Algebra>();
    println!("{}", algebra.unwrap().to_string());
}
