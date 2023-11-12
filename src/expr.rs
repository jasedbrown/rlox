#![allow(dead_code)]
use crate::token::Token;

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
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

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

    // fn visit(expr: Expr) -> Result<()> {
    //     use Expr::*;
    //     match expr {
    //         Assign(_t, _e) => (),
    //         Binary(_l, _t, _r) => (),
    //         Call(_e, _t, _v) => (),
    //         Get(_e, _t) => (),
    //         Grouping(_e) => (),
    //         Literal(_l) => (),
    //         Logical(_l, _t, _r) => (),
    //         Set(_l, _t, _r) => (),
    //         Super(_t1, _t2) => (),
    //         This(_t) => (),
    //         Unary(_t, _e) => (),
    //         Variable(_t) => (),
    //     };

    //     Ok(())
    // }
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
