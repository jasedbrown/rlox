#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};
use std::cell::RefCell;

use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

pub struct Interpreter<'a> {
    environment: RefCell<Environment<'a>>,
    _error_reporter: ErrorReporter,
}

impl<'a> Interpreter<'a> {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        Self {
            environment: RefCell::new(Environment::new(None)),
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
        match stmt {
            Stmt::Block(stmts) => {
                //                let p = self.environment.swap(RefCell::new(None));
                let local_env = RefCell::new(Environment::new(p));
                self.environment.swap(&local_env);
                // NOTE: local_env now ref's the parent env after this point.

                for stmt in stmts {
                    match self.execute(stmt) {
                        Ok(_) => (),
                        Err(e) => {
                            // restore the parent env on error
                            self.environment.swap(&local_env);
                            return Err(anyhow!("Error processing stmt: {:?}", e));
                        }
                    }
                }

                // restore the parent env
                self.environment.swap(&local_env);
                Ok(())
            }
            Stmt::Expression(e) => {
                let val = self.evaluate_expr(e)?;
                println!("-> {}", val);
                Ok(())
            }
            Stmt::Print(e) => {
                let val = self.evaluate_expr(e)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(e) => Some(self.evaluate_expr(e)?),
                    None => None,
                };
                self.environment.borrow().define(name.lexeme.clone(), val);
                Ok(())
            }
            _ => Err(anyhow!("unsupported stmt type: {:?}", stmt)),
        }
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<RlValue> {
        use Expr::*;
        match expr {
            Assign(t, e) => {
                let value = self.evaluate_expr(e)?;
                self.environment.borrow().assign(t, value.clone())?;
                Ok(value)
            }
            Binary(l, t, r) => {
                let left = self.evaluate_expr(l)?;
                let right = self.evaluate_expr(r)?;

                match t.token_type {
                    TokenType::Minus => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        let ret_val: f64 = left_d - right_d;
                        Ok(RlValue::Double(ret_val))
                    }
                    TokenType::Slash => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        let ret_val: f64 = left_d / right_d;
                        Ok(RlValue::Double(ret_val))
                    }
                    TokenType::Star => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        let ret_val: f64 = left_d * right_d;
                        Ok(RlValue::Double(ret_val))
                    }
                    TokenType::Plus => {
                        // TODO: there's a way to do this with match ...
                        if left.is_string() && right.is_string() {
                            let mut ss = left.as_string().expect("must be string").clone();
                            ss.push_str(right.as_string().expect("must be string").as_str());
                            Ok(RlValue::String(ss))
                        } else if left.is_numeric() && right.is_numeric() {
                            let d = left.as_numeric().expect("nust be numeric")
                                + right.as_numeric().expect("Must be numeric");
                            Ok(RlValue::Double(d))
                        } else {
                            Err(anyhow!("mismatched types for '+' operator"))
                        }
                    }
                    TokenType::Greater => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d > right_d))
                    }
                    TokenType::GreaterEqual => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d >= right_d))
                    }
                    TokenType::Less => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d < right_d))
                    }
                    TokenType::LessEqual => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d <= right_d))
                    }
                    TokenType::BangEqual => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d != right_d))
                    }
                    TokenType::EqualEqual => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        Ok(RlValue::Boolean(left_d == right_d))
                    }
                    _ => Err(anyhow!("should be unreachable :(")),
                }
            }
            Call(_e, _t, _v) => Ok(RlValue::Nil),
            Get(_e, _t) => Ok(RlValue::Nil),
            Grouping(e) => self.evaluate_expr(e.as_ref()),
            Literal(l) => Ok(RlValue::from(l)),
            Logical(_l, _t, _r) => Ok(RlValue::Nil),
            Set(_l, _t, _r) => Ok(RlValue::Nil),
            Super(_t1, _t2) => Ok(RlValue::Nil),
            This(_t) => Ok(RlValue::Nil),
            Unary(t, e) => {
                let right = self.evaluate_expr(e)?;
                match t.token_type {
                    TokenType::Minus => match right.as_numeric() {
                        Some(d) => Ok(RlValue::Double(-d)),
                        None => Err(anyhow!("rlvalue not a numeric value")),
                    },
                    TokenType::Bang => {
                        let b = !right.is_truthy();
                        Ok(RlValue::Boolean(b))
                    }
                    _ => Err(anyhow!("shouldn't get here!! ....")),
                }
            }
            Variable(t) => match self.environment.borrow().get(t)? {
                Some(rlvalue) => Ok(rlvalue),
                None => Ok(RlValue::Nil),
            },
        }
    }
}
