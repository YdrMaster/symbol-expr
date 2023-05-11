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

impl ToAlgebra for i64 {
    fn to_algebra(self) -> Algebra {
        unit::UnitExpr::Integer(self).to_algebra()
    }
}

impl ToAlgebra for String {
    fn to_algebra(self) -> Algebra {
        unit::UnitExpr::Variable(self).to_algebra()
    }
}
