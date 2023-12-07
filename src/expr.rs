#![allow(dead_code)]

use crate::token::Token;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Logical(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    Super(Token, Token),
    This(Token),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

#[derive(Clone, Debug)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil(),
}

impl Hash for LiteralValue {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            LiteralValue::String(s) => s.hash(state),
            // this is complete bullshit, but as f64 doesn't support hash,
            // stoopidly casting to i64 gets us through for this application.
            LiteralValue::Number(n) => ((n * 1000000.0) as i64).hash(state),
            LiteralValue::Boolean(b) => b.hash(state),
            LiteralValue::Nil() => 0.hash(state),
        }
    }
}

impl Eq for LiteralValue {}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::String(s), LiteralValue::String(o)) => s == o,
            // this is slightly half-assed, directly comparing two f64's,
            // but good enough for this application.
            (LiteralValue::Number(s), LiteralValue::Number(o)) => s == o,
            (LiteralValue::Boolean(s), LiteralValue::Boolean(o)) => s == o,
            (LiteralValue::Nil(), LiteralValue::Nil()) => true,
            _ => false,
        }
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil() => write!(f, "nil"),
        }
    }
}
impl Expr {
    fn sorta_pretty_print(expr: &Expr) -> String {
        use Expr::*;
        match expr {
            Assign(_t, _e) => String::new(),
            Binary(l, t, r) => Self::parenthesize(Some(&t.lexeme), vec![l, r]),
            Call(_e, _t, _v) => String::new(),
            Get(_e, _t) => String::new(),
            Grouping(e) => Self::parenthesize(None, vec![e]),
            Literal(l) => format!("{}", l),
            Logical(_l, _t, _r) => String::new(),
            Set(_l, _t, _r) => String::new(),
            Super(_t1, _t2) => String::new(),
            This(_t) => String::new(),
            Unary(t, e) => Self::parenthesize(Some(&t.lexeme), vec![e]),
            Variable(_t) => String::new(),
        }
    }

    fn parenthesize(name: Option<&str>, exprs: Vec<&Expr>) -> String {
        let mut s = String::new();
        s.push('(');

        if let Some(n) = name {
            s.push_str(n);
        }

        for e in exprs {
            s.push(' ');
            s.push_str(Self::sorta_pretty_print(e).as_str());
        }

        s.push(')');

        s
    }
}

#[cfg(test)]
mod test {
    use crate::expr::{Expr, LiteralValue};
    use crate::token::{Token, TokenType};

    #[test]
    fn simple_literal() {
        let s = Expr::Literal(LiteralValue::String("asdf".to_string()));
        println!("{:?}", Expr::sorta_pretty_print(&s));
    }

    #[test]
    fn simple_negative() {
        let n = Box::new(Expr::Literal(LiteralValue::Number(42.0)));
        let neg = Token::simple_token(TokenType::Bang, "!".to_string(), 0);
        let unary = Expr::Unary(neg, n);
        println!("{:?}", Expr::sorta_pretty_print(&unary));
    }

    #[test]
    fn simple_add() {
        let left = Box::new(Expr::Literal(LiteralValue::Number(42.0)));
        let right = Box::new(Expr::Literal(LiteralValue::Number(3.0)));
        let plus = Token::simple_token(TokenType::Plus, "+".to_string(), 0);
        let binary = Expr::Binary(left, plus, right);
        println!("{:?}", Expr::sorta_pretty_print(&binary));
    }
}
