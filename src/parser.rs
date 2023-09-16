use crate::arctic_ast::{ASTNode,Operation};
use pest::{self, Parser, pratt_parser::PrattParser, iterators::{Pairs, Pair}};


#[derive(pest_derive::Parser)]
#[grammar = "arctic.pest"]
pub struct ArcticParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(modulus, Left))
            .op(Op::infix(power, Left))
    };
}

pub fn parse(source: &str) -> Vec<ASTNode> { 
    let mut ast = vec![];
    match ArcticParser::parse(Rule::program, source) {
        Ok(mut pairs) => {
            for pair in pairs {
                match pair.as_rule() {
                    Rule::expr  => ast.push(parse_expr(pair.into_inner())),
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
    ast
}

fn parse_pair(pair: Pair<Rule>) -> ASTNode {
    match pair.as_rule() {
            Rule::num => ASTNode::Num(pair.as_str().parse::<i64>().unwrap()),
            Rule::expr => { parse_expr(pair.into_inner()) }
            Rule::ident => { ASTNode::Ident(pair.as_str().to_string()) },
            Rule::assign => {
                let mut pair = pair.into_inner();
                let ident = pair.next().unwrap();
                let expr = pair.next().unwrap();
                let expr = parse_pair(expr);
                ASTNode::Assign  {
                    ident: String::from(ident.as_str()),
                    expr: Box::new(expr),
                }
            }
            Rule::function => {
                let mut pair = pair.into_inner();
                let ident = pair.next().unwrap();
                let expr = pair.next().unwrap();
                let expr = parse_pair(expr);
                ASTNode::Function   {
                    ident: String::from(ident.as_str()),
                    arg: Box::new(expr),
                }
            },
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule)
        }
}

pub fn parse_expr(pairs: Pairs<Rule>) -> ASTNode {
    PRATT_PARSER
        .map_primary(|primary| parse_pair(primary))
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operation::Add,
                Rule::subtract => Operation::Subtract,
                Rule::multiply => Operation::Multiply,
                Rule::divide => Operation::Divide,
                Rule::modulus => Operation::Modulus,
                Rule::power => Operation::Power,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            ASTNode::BinaryOp { lhs: Box::new(lhs), op, rlh: Box::new(rhs) }
        })
        .parse(pairs)

}

