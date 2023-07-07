use std::io::{self, BufRead, Write};

use anyhow::Result;

mod error;
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

fn run(_input: &str) -> Result<()> {
    todo!();
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
        let _ = run(&buf);
        buf.clear();
    }
    Ok(())
}
