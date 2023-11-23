use std::env;
use std::process;

use anyhow::Result;
use rlox::{ErrorReporter, RLox};

fn main() -> Result<()> {
    let mut env_args: Vec<String> = env::args().collect();
    // ignore the first arg (it's the standard unix name of the process)
    env_args.remove(0);

    let error_reporter = ErrorReporter::default();
    let rlox = RLox::new(error_reporter.clone());

    match env_args.len() {
        0 => rlox.run_prompt()?,
        1 => rlox.run_file(&env_args[0])?,
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64);
        }
    };

    // Not sure if this is cool with the clone, but :shrug: for now
    if error_reporter.had_error() {
        process::exit(65);
    }

    Ok(())
}
