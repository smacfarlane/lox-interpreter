use std::io::{self, BufRead, Write};

use anyhow::Result;

mod ast;
mod data_types;
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

fn run(input: &str) -> Result<()> {
    let mut scanner = crate::scanner::Scanner::new(input.to_string());
    let tokens = dbg!(scanner.scan_tokens())?;

    let mut parser = crate::parser::Parser::new(tokens);
    let expr = dbg!(parser.parse())?;

    let result = crate::interpreter::interpret(&Box::new(expr))?;

    println!("{}", result);

    Ok(())
}

fn run_file(_filename: String) -> Result<()> {
    Ok(())
}

fn repl() -> Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("unable to flush stdout");
        let mut buf = String::new();
        let mut stdin = io::stdin().lock();

        let _ = stdin.read_line(&mut buf)?;

        if buf.trim().is_empty() {
            break;
        }
        run(&buf)?;
        buf.clear();
    }
    Ok(())
}
