extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "expr.pest"]
pub struct ExprParser;

fn main() {
    let algebra = "1+2*3/4%(a_+B*_c/d0)";
    let ans = ExprParser::parse(Rule::algebra, algebra).unwrap();
    to_ast(ans);
}

fn to_ast(mut expr: Pairs<Rule>) -> Algebra {
    let algebra = expr.next().unwrap();
    assert_eq!(expr.next(), None);
    assert_eq!(algebra.as_rule(), Rule::algebra);
    parse_algrbra(algebra)
}

fn parse_algrbra(expr: Pair<Rule>) -> Algebra {
    let mut expr = expr.into_inner();
    let sum = expr.next().unwrap();
    assert_eq!(expr.next(), None);
    assert_eq!(sum.as_rule(), Rule::sum_expr);

    let mut op = SumOp::Add;
    let mut ans = Vec::new();
    let mut sum = sum.into_inner().into_iter();
    if let Some(pair) = sum.next() {
        match pair.as_rule() {
            Rule::prod_expr => ans.push((op, parse_product(pair))),
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
                Rule::prod_expr => (op, parse_product(pair)),
                _ => unreachable!(),
            });
        }
    }
    Algebra(ans)
}

fn parse_product(expr: Pair<Rule>) -> ProdExpr {
    println!("{expr:?}");
    println!();
    ProdExpr(vec![])
}

struct Integer(i64);
struct Identifier(String);
enum UnaryOp {
    Negative,
}
#[derive(Clone, Copy, Debug)]
enum ProdOp {
    Mul,
    Div,
    Mod,
}
#[derive(Clone, Copy, Debug)]
enum SumOp {
    Add,
    Sub,
}
struct Getter {
    array: Identifier,
    index: Box<Algebra>,
}
enum UnitExpr {
    Integer,
    Identifier,
    Getter,
    Algebra,
}
enum UnaryExpr {
    Pos(UnitExpr),
    Neg(UnitExpr),
}
struct ProdExpr(Vec<(ProdOp, UnaryExpr)>);
struct Algebra(Vec<(SumOp, ProdExpr)>);
