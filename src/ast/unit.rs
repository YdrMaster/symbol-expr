use super::{Algebra, Rule};
use pest::iterators::Pair;

#[derive(Clone, Copy, Default, Debug)]
pub(super) struct Integer(i64);

#[derive(Clone, Debug)]
pub(super) struct Identifier(String);

#[derive(Clone, Debug)]
pub(super) struct Getter {
    array: Identifier,
    index: Algebra,
}

#[derive(Clone, Debug)]
pub(super) enum UnitExpr {
    Integer(Integer),
    Identifier(Identifier),
    Getter(Getter),
    Algebra(Algebra),
}

impl From<Pair<'_, Rule>> for UnitExpr {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut pair = value.into_inner();
        let unit = pair.next().unwrap();
        assert_eq!(pair.next(), None);
        match unit.as_rule() {
            Rule::integer => Self::Integer(Integer(unit.as_str().parse().unwrap())),
            Rule::identifier => Self::Identifier(Identifier(unit.as_str().to_string())),
            Rule::getter => todo!(),
            Rule::algebra => Self::Algebra(unit.into()),
            _ => unreachable!(),
        }
    }
}
