use super::{Algebra, Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Generator(Vec<Algebra>);

impl From<Pair<'_, Rule>> for Generator {
    fn from(value: Pair<'_, Rule>) -> Self {
        todo!()
    }
}
