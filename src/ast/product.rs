use super::{sum::SumExpr, unary::UnaryExpr, IntoAlgebra, Rule, SymbolExpr, ValueRepo};
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

impl IntoAlgebra for ProdExpr {
    fn into_algebra(self) -> super::Algebra {
        SumExpr::from(self).into_algebra()
    }
}

impl AsRef<str> for ProdOp {
    fn as_ref(&self) -> &str {
        match self {
            ProdOp::Mul => "*",
            ProdOp::Div => "/",
            ProdOp::Mod => "%",
        }
    }
}

impl ToString for ProdExpr {
    fn to_string(&self) -> String {
        assert!(matches!(self.0.first().unwrap().0, ProdOp::Mul));
        let mut ans = String::new();
        ans.push_str(self.0[0].1.to_string().as_str());
        for (op, expr) in &self.0[1..] {
            ans.push(' ');
            ans.push_str(op.as_ref());
            ans.push(' ');
            ans.push_str(expr.to_string().as_str());
        }
        ans
    }
}

impl SymbolExpr for ProdExpr {
    fn calculate(&self, repo: &impl ValueRepo) -> i64 {
        self.0.iter().fold(1i64, |prod, (op, expr)| match op {
            ProdOp::Mul => prod * expr.calculate(repo),
            ProdOp::Div => prod / expr.calculate(repo),
            ProdOp::Mod => prod % expr.calculate(repo),
        })
    }
}
