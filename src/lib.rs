pub(crate) mod callable;
pub(crate) mod environment;
pub(crate) mod error;
pub(crate) mod expr;
pub(crate) mod interpreter;
pub(crate) mod parser;
pub(crate) mod resolver;
pub(crate) mod rlvalue;
pub(crate) mod scanner;
pub(crate) mod stmt;
pub(crate) mod token;

use std::cell::Cell;
use std::fs;
use std::io::stdin;

use crate::error::Result;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

/// A centralized error reporting struct. Should be passed around to all
/// the workers in this project.
#[derive(Default, Clone, Debug)]
pub struct ErrorReporter {
    // TODO(jeb): Not sure if Cell is the best here, but it's
    // at least some form of interior mutability (yay!)
    had_error: Cell<bool>,
}

impl ErrorReporter {
    pub fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&self, line: u32, place: &str, message: &str) {
        println!("[line {:?}] Error {:?}: {:?}", line, place, message);
        self.had_error.replace(true);
    }

    pub fn reset(&self) {
        self.had_error.replace(false);
    }

    pub fn had_error(&self) -> bool {
        self.had_error.get()
    }
}

/// The main struct for doing all the things for this project.
pub struct RLox {
    interpreter: Interpreter,
    error_reporter: ErrorReporter,
}

impl RLox {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        RLox {
            interpreter: Interpreter::new(error_reporter.clone()),
            error_reporter,
        }
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        println!("JEB::run_prompt HEAD");
        loop {
            println!("\nenter some program: ");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed to read line");
            let input = input.as_str().trim();

            if input.is_empty() {
                return Ok(());
            }

            if let Err(e) = self.run(input) {
                println!("Erorr occurred: {:?}", e);
                return Err(e);
            }

            // reset the has_error on each run...
            self.error_reporter.reset();
        }
    }

    fn run(&mut self, input: &str) -> Result<()> {
        // 1. scan
        let mut scanner = Scanner::new(input.to_string(), self.error_reporter.clone());
        scanner.scan_tokens()?;
        let tokens = scanner.tokens();

        // 2. parse
        let mut parser = Parser::new(tokens, self.error_reporter.clone());
        let stmts = parser.parse()?;

        self.interpreter.interpret(stmts)?;
        Ok(())
    }

    pub fn run_file(&mut self, filename: &str) -> Result<()> {
        let s = fs::read_to_string(filename)?;
        self.run(s.as_str())
    }
}
