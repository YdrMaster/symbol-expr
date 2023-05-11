use super::{unary::UnaryExpr, Algebra, Rule, ToAlgebra};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub(super) enum UnitExpr {
    Integer(Integer),
    Variable(VarName),
    Getter(Getter),
    Algebra(Algebra),
}

impl From<Pair<'_, Rule>> for UnitExpr {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut pair = value.into_inner();
        let unit = pair.next().unwrap();
        assert_eq!(pair.next(), None);
        match unit.as_rule() {
            Rule::integer => Self::Integer(unit.into()),
            Rule::var_name => Self::Variable(unit.into()),
            Rule::getter => Self::Getter(unit.into()),
            Rule::algebra => Self::Algebra(unit.into()),
            _ => unreachable!(),
        }
    }
}

impl From<Integer> for UnitExpr {
    fn from(value: Integer) -> Self {
        Self::Integer(value)
    }
}

impl From<VarName> for UnitExpr {
    fn from(value: VarName) -> Self {
        Self::Variable(value)
    }
}

impl From<Getter> for UnitExpr {
    fn from(value: Getter) -> Self {
        Self::Getter(value)
    }
}

impl From<Algebra> for UnitExpr {
    fn from(value: Algebra) -> Self {
        Self::Algebra(value)
    }
}

impl ToAlgebra for UnitExpr {
    fn to_algebra(self) -> super::Algebra {
        UnaryExpr::from(self).to_algebra()
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Integer(i64);

impl From<Pair<'_, Rule>> for Integer {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().parse().unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct VarName(String);

impl From<Pair<'_, Rule>> for VarName {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().to_string())
    }
}

#[derive(Clone, Debug)]
pub struct ListName(String);

impl From<Pair<'_, Rule>> for ListName {
    fn from(value: Pair<'_, Rule>) -> Self {
        Self(value.as_str().to_string())
    }
}

#[derive(Clone, Debug)]
pub(super) struct Getter {
    array: ListName,
    index: Algebra,
}

impl From<Pair<'_, Rule>> for Getter {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut pair = value.into_inner();
        let array = pair.next().unwrap();
        let index = pair.next().unwrap();
        assert_eq!(pair.next(), None);
        assert_eq!(array.as_rule(), Rule::list_name);
        assert_eq!(index.as_rule(), Rule::algebra);
        Self {
            array: ListName(array.as_str().to_string()),
            index: index.into(),
        }
    }
}
