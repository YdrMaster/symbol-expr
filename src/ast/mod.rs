mod algebra;
mod array;
mod generator;
mod product;
mod sum;
mod unary;
mod unit;

pub use algebra::Algebra;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

pub trait IntoAlgebra {
    fn into_algebra(self) -> Algebra;
}

impl IntoAlgebra for i64 {
    fn into_algebra(self) -> Algebra {
        unit::UnitExpr::Integer(self).into_algebra()
    }
}

impl IntoAlgebra for String {
    fn into_algebra(self) -> Algebra {
        unit::UnitExpr::Variable(self).into_algebra()
    }
}

pub trait ValueRepo {
    fn get_value(&self, name: &str) -> i64;
    fn index_value(&self, name: &str, index: i64) -> i64;
}

pub trait SymbolExpr {
    fn calculate(&self, repo: &impl ValueRepo) -> i64;
}
