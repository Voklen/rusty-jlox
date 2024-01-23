use crate::{error, scanner::scanner};
use anyhow::{Context, Result};
use std::{
	env, fs,
	io::{self, Write},
	process,
	str::Chars,
};

pub fn get_passed_filename() -> Option<String> {
	let mut args = env::args().skip(1);
	let filename = args.next();
	if args.next() != None {
		println!("Usage: rjlox <file>");
		process::exit(64);
	};
	filename
}

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

pub fn run_file(filename: String) -> Result<()> {
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
