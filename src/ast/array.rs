use super::{Algebra, Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Array(Vec<Algebra>);

impl From<Pair<'_, Rule>> for Array {
    fn from(value: Pair<'_, Rule>) -> Self {
        todo!()
    }
}
