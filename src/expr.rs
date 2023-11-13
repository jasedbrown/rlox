#![allow(dead_code)]

use crate::rlvalue::RlValue;
use crate::token::{Token, TokenType};

use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
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

    pub(crate) fn evaluate(&self) -> Result<RlValue> {
        use Expr::*;
        match self {
            Assign(_t, _e) => Ok(RlValue::Nil),
            Binary(l, t, r) => {
                let left = l.evaluate()?;
                let right = r.evaluate()?;

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
            Call(_e, _t, _v) => Ok(RlValue::Nil),
            Get(_e, _t) => Ok(RlValue::Nil),
            Grouping(e) => e.as_ref().evaluate(),
            Literal(l) => Ok(RlValue::from(l)),
            Logical(_l, _t, _r) => Ok(RlValue::Nil),
            Set(_l, _t, _r) => Ok(RlValue::Nil),
            Super(_t1, _t2) => Ok(RlValue::Nil),
            This(_t) => Ok(RlValue::Nil),
            Unary(t, e) => {
                let right = e.evaluate()?;
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
            Variable(_t) => Ok(RlValue::Nil),
        }
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
