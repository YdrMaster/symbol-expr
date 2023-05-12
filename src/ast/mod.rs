mod algebra;
mod array;
mod generator;
mod product;
mod sum;
mod unary;
mod unit;

use pest::Parser;
use std::{matches, str::FromStr};

pub use algebra::Algebra;
pub use array::Array;
pub use generator::Generator;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct ExprParser;

#[derive(Clone, Debug)]
pub enum SymbolObj {
    Algebra(Algebra),
    Generator(Generator),
    Array(Array),
}

impl FromStr for SymbolObj {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = ExprParser::parse(Rule::obj, s).unwrap();
        let pair = pairs.next().unwrap();
        assert!(matches!(pair.as_rule(), Rule::obj));
        assert_eq!(pairs.next(), None);

        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        assert_eq!(pairs.next(), None);

        match pair.as_rule() {
            Rule::algebra => Ok(Self::Algebra(pair.into())),
            Rule::generator => Ok(Self::Generator(pair.into())),
            Rule::array => Ok(Self::Array(pair.into())),
            x => unreachable!("Unexpected rule: {x:?}"),
        }
    }
}

impl ToString for SymbolObj {
    fn to_string(&self) -> String {
        match self {
            Self::Algebra(algebra) => algebra.to_string(),
            Self::Generator(generator) => generator.to_string(),
            Self::Array(array) => array.to_string(),
        }
    }
}

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
