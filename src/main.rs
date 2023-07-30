use std::io::{self, BufRead, Write};

use crate::interpreter::Interpreter;

use anyhow::Result;

use tracing_flame::FlameLayer;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
};

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

    //let fmt_layer = fmt::Layer::default()
    //.with_span_events(FmtSpan::ENTER)
    //.pretty();

    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();

    tracing_subscriber::registry()
        //    .with(fmt_layer)
        .with(flame_layer)
        .init();

    let _ = match args.len() {
        1 => repl(),
        2 => run_file(args.nth(1).unwrap()),
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
    let statements = parser.parse()?;

    dbg!(interpreter.interpret(statements))?;

    Ok(())
}

fn run_file(filename: String) -> Result<()> {
    let mut interpreter = Interpreter::new();
    let file = std::fs::File::open(filename)?;

    //   for line in std::io::BufReader::new(file).lines() {
    //      dbg!(run(&mut interpreter, &line.unwrap()))?;
    // }
    //
    //
    let program = std::io::read_to_string(file).unwrap();
    dbg!(run(&mut interpreter, &program)?);

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
