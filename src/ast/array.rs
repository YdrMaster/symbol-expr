use super::{Algebra, Rule, SymbolList};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Array(Vec<Algebra>);

impl From<Pair<'_, Rule>> for Array {
    fn from(value: Pair<'_, Rule>) -> Self {
        let mut ans = Self(Vec::new());
        for pair in value.into_inner() {
            assert!(matches!(pair.as_rule(), Rule::algebra));
            ans.0.push(pair.into());
        }
        ans
    }
}

impl ToString for Array {
    fn to_string(&self) -> String {
        let mut ans = String::from("[");
        if !self.0.is_empty() {
            for algebra in &self.0 {
                ans.push_str(&algebra.to_string());
                ans.push_str(", ");
            }
            ans.pop();
            ans.pop();
        }
        ans.push(']');
        ans
    }
}

impl SymbolList for Array {
    fn get_symbol(&self, idx: usize) -> Algebra {
        self.0.get(idx).cloned().unwrap()
    }
}
