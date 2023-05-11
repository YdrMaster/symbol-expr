use super::{sum::SumExpr, ExprParser, Rule, ToAlgebra};
use pest::{iterators::Pair, Parser};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Algebra(SumExpr);

impl FromStr for Algebra {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = ExprParser::parse(Rule::algebra, s).unwrap();
        let pair = pairs.next().unwrap();
        assert_eq!(pairs.next(), None);
        assert_eq!(pair.as_rule(), Rule::algebra);
        Ok(Self::from(pair))
    }
}

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

impl ToAlgebra for Algebra {
    fn to_algebra(self) -> Algebra {
        self
    }
}
