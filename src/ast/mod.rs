mod algebra;
mod product;
mod sum;
mod unary;
mod unit;

pub use algebra::Algebra;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

pub trait ToAlgebra {
    fn to_algebra(self) -> Algebra;
}
