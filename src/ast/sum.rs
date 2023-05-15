use super::{product::ProdExpr, IntoAlgebra, Rule, SymbolExpr, ValueRepo};
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

impl From<ProdExpr> for SumExpr {
    fn from(value: ProdExpr) -> Self {
        Self(vec![(SumOp::Add, value)])
    }
}

impl IntoAlgebra for SumExpr {
    fn into_algebra(self) -> super::Algebra {
        self.into()
    }
}

impl AsRef<str> for SumOp {
    fn as_ref(&self) -> &str {
        match self {
            SumOp::Add => "+",
            SumOp::Sub => "-",
        }
    }
}

impl ToString for SumExpr {
    fn to_string(&self) -> String {
        assert!(matches!(self.0.first().unwrap().0, SumOp::Add));
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

impl SymbolExpr for SumExpr {
    fn substitute(&self, name: &str, val: i64) -> Self {
        Self(
            self.0
                .iter()
                .map(|(op, expr)| (*op, expr.substitute(name, val)))
                .collect(),
        )
    }

    fn calculate(&self, repo: &impl ValueRepo) -> i64 {
        self.0.iter().fold(1i64, |sum, (op, expr)| match op {
            SumOp::Add => sum + expr.calculate(repo),
            SumOp::Sub => sum - expr.calculate(repo),
        })
    }
}
