
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::enviroment::Env;
use crate::eval::Eval;
use crate::parser;

pub fn repl() -> Result<()> {
    let mut env = Env::new();
    let mut interpreter = Eval::new(env);
    let mut rl = DefaultEditor::new()?;

    println!("Arctic prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let ast = parser::parse(&line);
                for node in ast {
                    println!("{:?}", interpreter.eval(node))
                }

            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())

}