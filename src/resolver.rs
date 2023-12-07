use std::collections::HashMap;

use crate::error::{Result, RloxError};
use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,

    // CI uses a java.util.Stack<Map> to hold the scopes.
    // Rust has no stack data structure in the std lib,
    // but Vec is good enough as we only need push/pop.
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                self.begin_scope();
                for stmt in stmts {
                    self.resolve(stmt)?;
                }
                self.end_scope();
                Ok(())
            }
            Stmt::Expression(e) => {
                self.resolve_expr(e)?;
                Ok(())
            }
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
            } => {
                self.resolve_expr(condition)?;
                self.resolve(then_branch)?;
                if let Some(el) = else_branch {
                    self.resolve(el)?;
                }
                Ok(())
            }
            Stmt::Print(e) => {
                self.resolve_expr(e)?;
                Ok(())
            }
            Stmt::Return { expr, .. } => {
                if let Some(e) = expr {
                    self.resolve_expr(e)?;
                }
                Ok(())
            }
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
                self.resolve_expr(e)?;
                self.resolve_local(expr, t)?;
                Ok(())
            }
            Binary(l, t, r) => {
                self.resolve_expr(l)?;
                self.resolve_expr(r)?;
                Ok(())
            }
            Call(expr, _token, arguments) => {
                // TODO: double check this is correct.
                // CI references Expr::Call.callee
                self.resolve_expr(expr)?;
                for e in arguments {
                    self.resolve_expr(e)?;
                }
                Ok(())
            }
            // Get(_e, _t) => Ok(RlValue::Nil),
            Grouping(e) => {
                self.resolve_expr(e)?;
                Ok(())
            }
            Literal(..) => Ok(()),
            Logical(left, operator, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            // Set(_l, _t, _r) => Ok(RlValue::Nil),
            // Super(_t1, _t2) => Ok(RlValue::Nil),
            // This(_t) => Ok(RlValue::Nil),
            Unary(t, e) => {
                self.resolve_expr(e)?;
                Ok(())
            }
            Variable(t) => {
                match self.scopes.last_mut().and_then(|m| m.get(&t.lexeme)) {
                    Some(true) => (),
                    Some(false) | None => {
                        return Err(RloxError::ResolveError(String::from(
                            "Can't read local var in its own initializer",
                        )))
                    }
                };

                self.resolve_local(expr, t)
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
                let depth = (self.scopes.len() - 1 - i) as u32;
                self.interpreter.resolve(expr, depth)?;
            }

            i -= 1;
        }

        Ok(())
    }
}
