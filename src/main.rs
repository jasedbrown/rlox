use std::env;
use std::process;

pub(crate) mod callable;
pub(crate) mod environment;
pub(crate) mod error;
pub(crate) mod expr;
pub(crate) mod interpreter;
pub(crate) mod rlvalue;
pub(crate) mod stmt;
pub(crate) mod token;

use error::Result;
use rlox::{ErrorReporter, RLox};

fn main() -> Result<()> {
    let env_args: Vec<String> = env::args().collect();

    let error_reporter = ErrorReporter::default();
    let mut rlox = RLox::new(error_reporter.clone());

    match env_args.len() {
        // ignore the first arg (it's the standard unix name of the process)
        1 => rlox.run_prompt().unwrap(),
        2 => rlox.run_file(&env_args[1]).unwrap(),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    };

    // why the '?' operator requires type magic, I'm not sure,
    // just doing this shitty alternative ... :(

    // Not sure if this is cool with the clone, but :shrug: for now
    if error_reporter.had_error() {
        process::exit(65);
    }

    Ok(())
}
