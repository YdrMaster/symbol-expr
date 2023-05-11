mod product;
mod sum;
mod unary;
mod unit;

use pest::{iterators::Pair, Parser};
use std::str::FromStr;
use sum::SumExpr;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

#[derive(Clone, Debug)]
pub struct Algebra(SumExpr);

impl FromStr for Algebra {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = ExprParser::parse(Rule::algebra, s).unwrap();
        let algebra = pairs.next().unwrap();
        assert_eq!(pairs.next(), None);
        assert_eq!(algebra.as_rule(), Rule::algebra);
        Ok(Self::from(algebra))
    }
}

impl From<Pair<'_, Rule>> for Algebra {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut expr = value.into_inner();
        let sum = expr.next().unwrap();
        assert_eq!(expr.next(), None);
        assert_eq!(sum.as_rule(), Rule::sum_expr);
        Self(SumExpr::from(sum))
    }
}
