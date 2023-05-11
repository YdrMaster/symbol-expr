use super::{product::ProdExpr, Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub(super) struct SumExpr(Vec<(SumOp, ProdExpr)>);

#[derive(Clone, Copy, Debug)]
enum SumOp {
    Add,
    Sub,
}

impl From<Pair<'_, Rule>> for SumExpr {
    fn from(value: Pair<Rule>) -> Self {
        let mut op = SumOp::Add;
        let mut ans = Vec::new();
        let mut sum = value.into_inner();
        if let Some(pair) = sum.next() {
            match pair.as_rule() {
                Rule::prod_expr => ans.push((op, pair.into())),
                _ => unreachable!(),
            }
            loop {
                let Some(pair) = sum.next() else { break; };
                op = match pair.as_rule() {
                    Rule::sum_op => match pair.as_str() {
                        "+" => SumOp::Add,
                        "-" => SumOp::Sub,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                let pair = sum.next().unwrap();
                ans.push(match pair.as_rule() {
                    Rule::prod_expr => (op, pair.into()),
                    _ => unreachable!(),
                });
            }
        }
        Self(ans)
    }
}
