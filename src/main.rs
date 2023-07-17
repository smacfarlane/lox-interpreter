use std::io::{self, BufRead, Write};

use crate::interpreter::Interpreter;

use anyhow::Result;

mod ast;
mod data_types;
mod environment;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

fn main() {
    let mut args = std::env::args();

    let _ = match args.len() {
        1 => repl(),
        2 => run_file(args.next().unwrap()),
        _ => {
            println!("Usage: lox [script]");
            Ok(())
        }
    };
}

fn run(interpreter: &mut Interpreter, input: &str) -> Result<()> {
    let mut scanner = crate::scanner::Scanner::new(input.to_string());
    let tokens = scanner.scan_tokens()?;

    let mut parser = crate::parser::Parser::new(tokens);
    let statements = dbg!(parser.parse()?);

    interpreter.interpret(statements)?;

    Ok(())
}

fn run_file(_filename: String) -> Result<()> {
    Ok(())
}

fn repl() -> Result<()> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("unable to flush stdout");
        let mut buf = String::new();
        let mut stdin = io::stdin().lock();

        let _ = stdin.read_line(&mut buf)?;

        if buf.trim().is_empty() {
            break;
        }
        match run(&mut interpreter, &buf) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
        buf.clear();
    }
    Ok(())
}
