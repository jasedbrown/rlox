#![allow(dead_code)]

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class,
    Expression(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        // `Option` to allow a function to have no return value,
        // like a "void function" - for example, in Java:
        // public void doIt(String, int, ...) {}
        expr: Option<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
