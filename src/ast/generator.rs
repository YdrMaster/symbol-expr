use super::{Algebra, Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Generator {
    variable: String,
    func: Algebra,
}

impl From<Pair<'_, Rule>> for Generator {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut inner = value.into_inner();
        let variable = inner.next().unwrap();
        let func = inner.next().unwrap();
        assert_eq!(variable.as_rule(), Rule::idx_name);
        assert_eq!(func.as_rule(), Rule::algebra);
        assert_eq!(inner.next(), None);
        Self {
            variable: variable.as_str().to_string(),
            func: Algebra::from(func),
        }
    }
}

impl ToString for Generator {
    fn to_string(&self) -> String {
        format!("[{} -> {}]", self.variable, self.func.to_string())
    }
}
