#![allow(dead_code)]

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class,
    Expression(Expr),
    Function,
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Return,
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
