use std::collections::HashMap;

use crate::error::{Result, RloxError};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

struct Resolver<'a> {
    interpreter: &'a Interpreter,

    // CI uses a java.util.Stack<Map> to hold the scopes.
    // Rust has no stack data structure in the std lib,
    // but Vec is good enough as we only need push/pop.
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    fn resolve(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                self.begin_scope();
                for stmt in stmts {
                    self.resolve(stmt)?;
                }
                self.end_scope();
                Ok(())
            }
            Stmt::Expression(e) => Ok(()),
            Stmt::Function { name, params, body } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(params, body)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => Ok(()),
            Stmt::Print(e) => Ok(()),
            Stmt::Return { expr, .. } => Ok(()),
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(init) = initializer {
                    self.resolve_expr(init)?;
                }
                self.define(name);
                Ok(())
            }
            Stmt::While { condition, body } => Ok(()),
            _ => Err(RloxError::Unsupported(format!(
                "unsupported stmt type: {:?}",
                stmt
            ))),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn declare(&mut self, name: &Token) {
        self.scopes
            .last_mut()
            .and_then(|m| m.insert(name.lexeme.clone(), false));
    }

    fn define(&mut self, name: &Token) {
        self.scopes
            .last_mut()
            .and_then(|m| m.insert(name.lexeme.clone(), true));
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_function(&mut self, params: &[Token], body: &[Stmt]) -> Result<()> {
        self.begin_scope();

        for param in params {
            self.declare(param);
            self.define(param);
        }

        for stmt in body {
            self.resolve(stmt)?;
        }

        self.end_scope();
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        use Expr::*;
        match expr {
            Assign(t, e) => {
                self.resolve_expr(&e)?;
                self.resolve_local(&expr, &t)?;
                Ok(())
            }
            // Binary(l, t, r) => {
            //     let left = self.evaluate_expr(l)?;
            //     let right = self.evaluate_expr(r)?;

            //     match t.token_type {
            //         TokenType::Minus => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             let ret_val: f64 = left_d - right_d;
            //             Ok(RlValue::Double(ret_val))
            //         }
            //         TokenType::Slash => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             let ret_val: f64 = left_d / right_d;
            //             Ok(RlValue::Double(ret_val))
            //         }
            //         TokenType::Star => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             let ret_val: f64 = left_d * right_d;
            //             Ok(RlValue::Double(ret_val))
            //         }
            //         TokenType::Plus => {
            //             // TODO: there's a way to do this with match ...
            //             if left.is_string() && right.is_string() {
            //                 let mut ss = left.as_string().expect("must be string").clone();
            //                 ss.push_str(right.as_string().expect("must be string").as_str());
            //                 Ok(RlValue::String(ss))
            //             } else if left.is_numeric() && right.is_numeric() {
            //                 let d = left.as_numeric().expect("nust be numeric")
            //                     + right.as_numeric().expect("Must be numeric");
            //                 Ok(RlValue::Double(d))
            //             } else {
            //                 Err(RloxError::IncorrectType(format!(
            //                     "mismatched types for '+' operator, l: {:?}, , r: {:?}",
            //                     l, r
            //                 )))
            //             }
            //         }
            //         TokenType::Greater => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d > right_d))
            //         }
            //         TokenType::GreaterEqual => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d >= right_d))
            //         }
            //         TokenType::Less => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d < right_d))
            //         }
            //         TokenType::LessEqual => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d <= right_d))
            //         }
            //         TokenType::BangEqual => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d != right_d))
            //         }
            //         TokenType::EqualEqual => {
            //             let left_d = left.as_numeric().unwrap();
            //             let right_d = right.as_numeric().unwrap();
            //             Ok(RlValue::Boolean(left_d == right_d))
            //         }
            //         _ => Err(RloxError::Unreachable(format!(
            //             "unsupported Binary type: {:?}",
            //             t,
            //         ))),
            //     }
            // }
            // Call(expr, _token, arguments) => {
            //     let callee = self.evaluate_expr(expr)?;
            //     let mut function = match callee {
            //         RlValue::Callable(c) => c,
            //         _ => {
            //             return Err(RloxError::IncorrectType(format!(
            //                 "tried to call a function, but {:?} is not a function: {:?}",
            //                 expr, callee
            //             )))
            //         }
            //     };

            //     let mut args = Vec::new();
            //     for arg in arguments {
            //         args.push(self.evaluate_expr(arg)?);
            //     }

            //     // check for the correct number of arguments
            //     if args.len() != function.arity() {
            //         return Err(RloxError::ArityError(function.arity(), args.len()));
            //     }

            //     Ok(function.call(self, &args)?)
            // }
            // Get(_e, _t) => Ok(RlValue::Nil),
            // Grouping(e) => self.evaluate_expr(e.as_ref()),
            // Literal(l) => Ok(RlValue::from(l)),
            // Logical(left, operator, right) => {
            //     let l = self.evaluate_expr(left)?;
            //     if operator.token_type == TokenType::Or {
            //         if l.is_truthy() {
            //             return Ok(l);
            //         }
            //     } else {
            //         if !l.is_truthy() {
            //             return Ok(l);
            //         }
            //     }

            //     self.evaluate_expr(right)
            // }
            // Set(_l, _t, _r) => Ok(RlValue::Nil),
            // Super(_t1, _t2) => Ok(RlValue::Nil),
            // This(_t) => Ok(RlValue::Nil),
            // Unary(t, e) => {
            //     let right = self.evaluate_expr(e)?;
            //     match t.token_type {
            //         TokenType::Minus => match right.as_numeric() {
            //             Some(d) => Ok(RlValue::Double(-d)),
            //             None => Err(RloxError::IncorrectType(String::from(
            //                 "rlvalue not a numeric value",
            //             ))),
            //         },
            //         TokenType::Bang => {
            //             let b = !right.is_truthy();
            //             Ok(RlValue::Boolean(b))
            //         }
            //         _ => Err(RloxError::Unreachable(format!(
            //             "TokenType not accepted: {:?}",
            //             t.token_type,
            //         ))),
            //     }
            // }
            Variable(t) => {
                match self.scopes.last_mut().and_then(|m| m.get(&t.lexeme)) {
                    Some(true) => (),
                    Some(false) | None => {
                        return Err(RloxError::ResolveError(String::from(
                            "Can't read local var in its own initializer",
                        )))
                    }
                };

                self.resolve_local(&expr, &t)
            }
            _ => Err(RloxError::ResolveError(format!(
                "not resolving this type of expr: {:?}",
                expr
            ))),
        }
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) -> Result<()> {
        let mut i = self.scopes.len() - 1;
        while i >= 0 {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i)?;
            }

            i -= 1;
        }

        Ok(())
    }
}
