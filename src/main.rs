#![allow(unused)]

use eval::Eval;

use crate::enviroment::Env;

mod parser;
mod arctic_ast;
mod eval;
mod enviroment;
mod repl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        repl::repl();
    }

}
