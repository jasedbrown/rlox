#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::Callable;
use crate::environment::Environment;
use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    env_id: RefCell<i32>,
    _error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        let env_id: i32 = 0;
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None, env_id))),
            env_id: RefCell::new(env_id),
            _error_reporter: error_reporter,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    // TODO: see if the 'mut' can be eliminated here - it's only used for changing the
    // environment
    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                let restore_env = Rc::clone(&self.environment);

                let outer_env = Rc::clone(&self.environment);

                self.env_id.replace_with(|&mut prev| prev + 1);
                self.environment = Rc::new(RefCell::new(Environment::new(
                    Some(outer_env),
                    *self.env_id.borrow(),
                )));
                //                self.environment.swap(&inner_env);

                for stmt in stmts {
                    match self.execute(stmt) {
                        Ok(_) => (),
                        Err(e) => {
                            // restore the parent env on error
                            self.environment.swap(&restore_env);
                            return Err(anyhow!("Error processing stmt: {:?}", e));
                        }
                    }
                }

                // restore the parent
                self.environment.swap(&restore_env);
                Ok(())
            }
            Stmt::Expression(e) => {
                self.evaluate_expr(e)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.evaluate_expr(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(el) = else_branch {
                    self.execute(el)?;
                }
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
                self.environment.borrow().define(name.clone(), val);
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate_expr(condition)?.is_truthy() {
                    self.execute(body)?;
                }
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
            Call(expr, _token, arguments) => {
                let callee = self.evaluate_expr(expr)?;
                let function = self.callable_lookup(&callee)?;

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expr(arg)?);
                }

                // check for the correct number of arguments
                if args.len() != function.arity() {
                    return Err(anyhow!(
                        "Expected {} args, but got {}",
                        function.arity(),
                        args.len()
                    ));
                }

                Ok(function.call(self, &args)?)
            }
            Get(_e, _t) => Ok(RlValue::Nil),
            Grouping(e) => self.evaluate_expr(e.as_ref()),
            Literal(l) => Ok(RlValue::from(l)),
            Logical(left, operator, right) => {
                let l = self.evaluate_expr(left)?;
                if operator.token_type == TokenType::Or {
                    if l.is_truthy() {
                        return Ok(l);
                    }
                } else {
                    if !l.is_truthy() {
                        return Ok(l);
                    }
                }

                self.evaluate_expr(right)
            }
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

    fn callable_lookup(&self, callee: &RlValue) -> Result<Callable> {
        let fn_name = match callee {
            RlValue::String(s) => s,
            _ => return Err(anyhow!("callee is not a string: {:?}", callee)),
        };

        // look up the funtion definition. first check as a built-in.
        if let Some(f) = Callable::find_builtin(fn_name) {
            return Ok(f);
        }

        Err(anyhow!("No function by name {:?} was found", fn_name))
    }
}
