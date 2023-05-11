use super::{sum::SumExpr, unary::UnaryExpr, Rule, ToAlgebra};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub(super) struct ProdExpr(Vec<(ProdOp, UnaryExpr)>);

#[derive(Clone, Copy, Debug)]
enum ProdOp {
    Mul,
    Div,
    Mod,
}

impl From<Pair<'_, Rule>> for ProdExpr {
    fn from(value: Pair<Rule>) -> Self {
        let mut op = ProdOp::Mul;
        let mut ans = Vec::new();
        let mut product = value.into_inner();
        if let Some(pair) = product.next() {
            match pair.as_rule() {
                Rule::unary_expr => ans.push((op, pair.into())),
                _ => unreachable!(),
            }
            loop {
                let Some(pair) = product.next() else { break; };
                op = match pair.as_rule() {
                    Rule::prod_op => match pair.as_str() {
                        "*" => ProdOp::Mul,
                        "/" => ProdOp::Div,
                        "%" => ProdOp::Mod,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };

                let pair = product.next().unwrap();
                ans.push(match pair.as_rule() {
                    Rule::unary_expr => (op, pair.into()),
                    _ => unreachable!(),
                });
            }
        }
        Self(ans)
    }
}

impl From<UnaryExpr> for ProdExpr {
    fn from(value: UnaryExpr) -> Self {
        Self(vec![(ProdOp::Mul, value)])
    }
}

impl ToAlgebra for ProdExpr {
    fn to_algebra(self) -> super::Algebra {
        SumExpr::from(self).to_algebra()
    }
}
