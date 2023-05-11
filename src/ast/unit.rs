use super::{unary::UnaryExpr, Algebra, Rule, ToAlgebra};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub(super) enum UnitExpr {
    Integer(i64),
    Variable(String),
    Getter { list: String, idx: Algebra },
    Algebra(Algebra),
}

impl From<Pair<'_, Rule>> for UnitExpr {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut pair = value.into_inner();
        let unit = pair.next().unwrap();
        assert_eq!(pair.next(), None);
        match unit.as_rule() {
            Rule::integer => Self::Integer(unit.as_str().parse().unwrap()),
            Rule::var_name => Self::Variable(unit.as_str().into()),
            Rule::getter => {
                let mut pair = unit.into_inner();
                let array = pair.next().unwrap();
                let index = pair.next().unwrap();
                assert_eq!(pair.next(), None);
                assert_eq!(array.as_rule(), Rule::list_name);
                assert_eq!(index.as_rule(), Rule::algebra);
                Self::Getter {
                    list: array.as_str().to_string(),
                    idx: index.into(),
                }
            }
            Rule::algebra => Self::Algebra(unit.into()),
            _ => unreachable!(),
        }
    }
}

impl ToAlgebra for UnitExpr {
    fn to_algebra(self) -> super::Algebra {
        UnaryExpr::from(self).to_algebra()
    }
}
