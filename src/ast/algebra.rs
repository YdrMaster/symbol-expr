﻿use super::{sum::SumExpr, ExprParser, IntoAlgebra, Rule, SymbolExpr, ValueRepo};
use pest::{iterators::Pair, Parser};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Algebra(SumExpr);

impl FromStr for Algebra {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = ExprParser::parse(Rule::algebra, s).unwrap();

        let pair = pairs.next().unwrap();
        assert!(matches!(pair.as_rule(), Rule::algebra));
        assert_eq!(pairs.next(), None);

        let pair = pair.into_inner().next().unwrap();
        assert!(matches!(pair.as_rule(), Rule::sum_expr));
        assert_eq!(pairs.next(), None);

        Ok(Self(pair.into()))
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
    fn substitute(&self, name: &str, val: i64) -> Self {
        Self(self.0.substitute(name, val))
    }

    fn calculate(&self, repo: &impl ValueRepo) -> i64 {
        self.0.calculate(repo)
    }
}
