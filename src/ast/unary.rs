use super::{product::ProdExpr, unit::UnitExpr, IntoAlgebra, Rule, SymbolExpr, ValueRepo};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub(super) struct UnaryExpr {
    ops: Vec<UnaryOp>,
    expr: UnitExpr,
}

#[derive(Clone, Copy, Debug)]
enum UnaryOp {
    Neg,
}

impl From<Pair<'_, Rule>> for UnaryExpr {
    fn from(value: Pair<Rule>) -> Self {
        let mut ans = Self {
            ops: Vec::new(),
            expr: UnitExpr::Integer(Default::default()),
        };
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::unary_op => match pair.as_str() {
                    "-" => ans.ops.push(UnaryOp::Neg),
                    _ => unreachable!(),
                },
                Rule::unit_expr => ans.expr = pair.into(),
                _ => unreachable!(),
            }
        }
        ans
    }
}

impl From<UnitExpr> for UnaryExpr {
    fn from(value: UnitExpr) -> Self {
        Self {
            ops: Vec::new(),
            expr: value,
        }
    }
}

impl IntoAlgebra for UnaryExpr {
    fn into_algebra(self) -> super::Algebra {
        ProdExpr::from(self).into_algebra()
    }
}

impl AsRef<str> for UnaryOp {
    fn as_ref(&self) -> &str {
        match self {
            UnaryOp::Neg => "-",
        }
    }
}

impl ToString for UnaryExpr {
    fn to_string(&self) -> String {
        let mut ans = String::new();
        for op in &self.ops {
            ans.push_str(op.as_ref())
        }
        ans.push_str(&self.expr.to_string());
        ans
    }
}

impl SymbolExpr for UnaryExpr {
    fn substitute(&self, name: &str, val: i64) -> Self {
        Self {
            ops: self.ops.clone(),
            expr: self.expr.substitute(name, val),
        }
    }

    fn calculate(&self, repo: &impl ValueRepo) -> i64 {
        let abs = self.expr.calculate(repo);
        if self.ops.len() % 2 == 0 {
            abs
        } else {
            -abs
        }
    }
}
