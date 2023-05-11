use super::{sum::SumExpr, IntoAlgebra, Rule, SymbolExpr, ValueRepo};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Algebra(SumExpr);

impl From<Pair<'_, Rule>> for Algebra {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut expr = value.into_inner();
        let sum = expr.next().unwrap();
        assert_eq!(expr.next(), None);
        assert_eq!(sum.as_rule(), Rule::sum_expr);
        Self(sum.into())
    }
}

impl From<SumExpr> for Algebra {
    fn from(value: SumExpr) -> Self {
        Self(value)
    }
}

impl IntoAlgebra for Algebra {
    fn into_algebra(self) -> Algebra {
        self
    }
}

impl ToString for Algebra {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl SymbolExpr for Algebra {
    fn calculate(&self, repo: &impl ValueRepo) -> i64 {
        self.0.calculate(repo)
    }
}
