use anyhow::Result;
use run::{run_file, run_repl};
use std::env;

mod run;

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
