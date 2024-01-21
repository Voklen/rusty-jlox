use anyhow::{Context, Result};
use std::{
    fs,
    io::{self, Write},
    str::Chars,
};

use crate::{error, scanner::scanner};

pub fn run_repl() -> Result<()> {
    print_prompt()?;
    let lines = io::stdin().lines();
    for line in lines {
        let err = run(line?.chars());
        match err {
            Ok(()) => {}
            Err(err) => error!("{err}"),
        }
        print_prompt()?;
    }
    Ok(())
}

fn print_prompt() -> Result<()> {
    print!("> ");
    io::stdout().flush().context("Error flushing stdout")
}

pub fn run_file(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename).context("Error opening lox file")?;
    run(contents.chars())
}

fn run(chars: Chars) -> Result<()> {
    let tokens = scanner(chars)?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
