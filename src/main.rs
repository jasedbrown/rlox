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
    let mut env_args: Vec<String> = env::args().collect();
    // ignore the first arg (it's the standard unix name of the process)
    env_args.remove(0);

    let error_reporter = ErrorReporter::default();
    let mut rlox = RLox::new(error_reporter.clone());

    let res = match env_args.len() {
        0 => rlox.run_prompt(),
        1 => rlox.run_file(&env_args[0]),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    };

    // why the '?' operator requires type magic, I'm not sure,
    // just doing this shitty alternative ... :(
    res.unwrap();

    // Not sure if this is cool with the clone, but :shrug: for now
    if error_reporter.had_error() {
        process::exit(65);
    }

    Ok(())
}
