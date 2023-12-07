#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::callable::Callable;
use crate::environment::{self, Environment};
use crate::error::{Result, RloxError};
use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

pub struct Interpreter {
    /// Top-most environment for holding, appropriately enough,
    /// global variables and functions.
    globals: Rc<RefCell<Environment>>,

    /// The current-level of enviroment for variables, functions,
    /// and so on.
    environment: Rc<RefCell<Environment>>,

    /// A debugging aid for quickly and simply identifying a
    /// given `Environment`.
    env_id: RefCell<i32>,

    /// For resolved variables, the distance from local context
    /// to where it's defined.
    locals: RefCell<HashMap<Expr, u32>>,

    _error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        let env_id: i32 = 0;
        let globals = Rc::new(RefCell::new(Environment::new(None, env_id)));
        let environment = globals.clone();
        Self {
            globals,
            environment,
            env_id: RefCell::new(env_id),
            locals: RefCell::new(HashMap::new()),
            _error_reporter: error_reporter,
        }
    }

    pub fn new_env_from_globals(&self) -> Environment {
        let outer_env = Rc::clone(&self.globals);
        self.env_id.replace_with(|&mut prev| prev + 1);
        Environment::new(Some(outer_env), *self.env_id.borrow())
    }

    pub fn new_env(&self) -> Environment {
        let outer_env = Rc::clone(&self.environment);
        self.env_id.replace_with(|&mut prev| prev + 1);
        Environment::new(Some(outer_env), *self.env_id.borrow())
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
                let env = self.new_env();
                self.execute_block(stmts, env)?;
                Ok(())
            }
            Stmt::Expression(e) => {
                self.evaluate_expr(e)?;
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                // TODO: not sure if i really need to clone() all the things ...
                let callable = Callable::Dynamic {
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.environment),
                };
                let rlcallable = Some(RlValue::Callable(callable));
                self.environment.borrow().define(name.clone(), rlcallable);
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
            Stmt::Return { expr, .. } => {
                let ret = match expr {
                    Some(expr) => Some(self.evaluate_expr(expr)?),
                    None => None,
                };

                // holy shit, the CI book will throw an exception (in Java)
                // in order to (early) return from a function.
                Err(RloxError::Return(ret))
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
            _ => Err(RloxError::Unsupported(format!(
                "unsupported stmt type: {:?}",
                stmt
            ))),
        }
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<RlValue> {
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
                            Err(RloxError::IncorrectType(format!(
                                "mismatched types for '+' operator, l: {:?}, , r: {:?}",
                                l, r
                            )))
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
                    _ => Err(RloxError::Unreachable(format!(
                        "unsupported Binary type: {:?}",
                        t,
                    ))),
                }
            }
            Call(expr, _token, arguments) => {
                let callee = self.evaluate_expr(expr)?;
                let mut function = match callee {
                    RlValue::Callable(c) => c,
                    _ => {
                        return Err(RloxError::IncorrectType(format!(
                            "tried to call a function, but {:?} is not a function: {:?}",
                            expr, callee
                        )))
                    }
                };

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expr(arg)?);
                }

                // check for the correct number of arguments
                if args.len() != function.arity() {
                    return Err(RloxError::ArityError(function.arity(), args.len()));
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
                        None => Err(RloxError::IncorrectType(String::from(
                            "rlvalue not a numeric value",
                        ))),
                    },
                    TokenType::Bang => {
                        let b = !right.is_truthy();
                        Ok(RlValue::Boolean(b))
                    }
                    _ => Err(RloxError::Unreachable(format!(
                        "TokenType not accepted: {:?}",
                        t.token_type,
                    ))),
                }
            }
            Variable(t) => match self.locals.borrow().get(expr) {
                Some(depth) => Ok(self.environment.borrow().get_at(*depth, t)?),
                None => Ok(self.globals.borrow().get(t)?.unwrap()),
            },
        }
    }

    pub fn execute_block(&mut self, stmts: &[Stmt], env: Environment) -> Result<RlValue> {
        let restore_env = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(env));
        let mut ret = Ok(RlValue::Nil);
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(RloxError::Return(val)) => {
                    if let Some(val) = val {
                        ret = Ok(val);
                    }
                    break;
                }
                Err(e) => {
                    ret = Err(e);
                    break;
                }
            }
        }

        // restore the parent
        self.environment = restore_env;
        ret
    }

    pub fn resolve(&self, expr: &Expr, depth: u32) -> Result<()> {
        self.locals.borrow_mut().insert(expr.clone(), depth);
        Ok(())
    }
}
