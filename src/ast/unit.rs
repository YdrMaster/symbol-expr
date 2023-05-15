use super::{unary::UnaryExpr, Algebra, IntoAlgebra, Rule, SymbolExpr};
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

impl IntoAlgebra for UnitExpr {
    fn into_algebra(self) -> super::Algebra {
        UnaryExpr::from(self).into_algebra()
    }
}

impl ToString for UnitExpr {
    fn to_string(&self) -> String {
        match self {
            UnitExpr::Integer(x) => x.to_string(),
            UnitExpr::Variable(n) => n.to_string(),
            UnitExpr::Getter { list, idx } => format!("{list}[{}]", idx.to_string()),
            UnitExpr::Algebra(a) => format!("({})", a.to_string()),
        }
    }
}

impl SymbolExpr for UnitExpr {
    fn substitute(&self, name: &str, val: i64) -> Self {
        match self {
            Self::Variable(n) if n == name => Self::Integer(val),
            Self::Integer(_) | Self::Variable(_) => self.clone(),
            Self::Getter { list, idx } => Self::Getter {
                list: list.clone(),
                idx: idx.substitute(name, val),
            },
            Self::Algebra(a) => Self::Algebra(a.substitute(name, val)),
        }
    }

    fn calculate(&self, repo: &impl super::ValueRepo) -> i64 {
        match self {
            UnitExpr::Integer(x) => *x,
            UnitExpr::Variable(n) => repo.get_value(n),
            UnitExpr::Getter { list, idx } => repo.index_value(list, idx.calculate(repo)),
            UnitExpr::Algebra(a) => a.calculate(repo),
        }
    }
}
