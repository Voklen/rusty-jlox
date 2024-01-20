use std::{
    env, fs,
    io::{self, Write},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let filename = match args.next() {
        Some(filename) => filename,
        None => return run_repl(),
    };
    if args.next().is_some() {
        println!("Usage: rjlox <file>");
        return Ok(());
    }
    run_file(&filename)
}

fn run_repl() -> Result<()> {
    print_prompt()?;
    let lines = io::stdin().lines();
    for line in lines {
        run(line?)?;
        print_prompt()?;
    }
    Ok(())
}

fn print_prompt() -> Result<()> {
    print!("> ");
    io::stdout().flush().context("Error flushing stdout")
}

fn run_file(filename: &str) -> Result<()> {
    let contents = fs::read_to_string(filename).context("Error opening lox file")?;
    run(contents)
}

fn run(string: String) -> Result<()> {
    println!("{string}");
    Ok(())
}
