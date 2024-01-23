use anyhow::Result;
use run::{get_passed_filename, run_file, run_repl};

mod errors;
mod run;
mod scanner;

fn main() -> Result<()> {
    if let Some(filename) = get_passed_filename() {
        run_file(filename)
    } else {
        run_repl()
    }
}
