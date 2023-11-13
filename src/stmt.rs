#![allow(dead_code)]

use crate::expr::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block,
    Class,
    Expression(Expr),
    Function,
    If,
    Print(Expr),
    Return,
    Var,
    While,
}
