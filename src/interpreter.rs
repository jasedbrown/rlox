#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    _error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        Self {
            environment: Default::default(),
            _error_reporter: error_reporter,
        }
    }

    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&self, stmt: &Stmt) -> Result<()> {
        use Stmt::*;
        match stmt {
            Stmt::Print(e) => {
                let val = self.evaluate(e)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::Expression(e) => {
                let _ = self.evaluate(e)?;
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(e) => Some(self.evaluate(e)?),
                    None => None,
                };
                self.environment.define(name.lexeme.clone(), val);
                Ok(())
            }
            _ => Err(anyhow!("unsupported stmt type: {:?}", stmt)),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<RlValue> {
        expr.evaluate()
    }
}
