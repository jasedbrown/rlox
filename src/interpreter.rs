#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

pub struct Interpreter {
    _error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        Self {
            _error_reporter: error_reporter,
        }
    }

    pub fn interpret(stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts.iter() {
            Self::execute(stmt)?;
        }
        Ok(())
    }

    fn execute(stmt: &Stmt) -> Result<()> {
        use Stmt::*;
        match stmt {
            Stmt::Print(e) => {
                let val = Self::evaluate(e)?;
                println!("{}", val);
                return Ok(());
            }
            Stmt::Expression(e) => {
                let _ = Self::evaluate(e)?;
                return Ok(());
            }
            _ => Err(anyhow!("unsupported stmt type: {:?}", stmt)),
        }
    }

    fn evaluate(expr: &Expr) -> Result<RlValue> {
        Ok(expr.evaluate()?)
    }

    //    fn print_stmt(expr:
}
