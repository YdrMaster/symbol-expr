mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use ast::SymbolObj;

fn main() {
    let algebra = "[1,2,3]".parse::<SymbolObj>();
    println!("{}", algebra.unwrap().to_string());
}
