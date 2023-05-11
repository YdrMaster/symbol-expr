﻿use super::{product::ProdExpr, unit::UnitExpr, Rule, ToAlgebra};
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

impl ToAlgebra for UnaryExpr {
    fn to_algebra(self) -> super::Algebra {
        ProdExpr::from(self).to_algebra()
    }
}
