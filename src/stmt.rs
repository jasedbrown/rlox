#![allow(dead_code)]

use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class,
    Expression(Expr),
    Function,
    If,
    Print(Expr),
    Return,
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While,
}
