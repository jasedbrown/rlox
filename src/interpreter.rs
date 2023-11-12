#![allow(dead_code, unused_imports)]

use anyhow::{anyhow, Result};

use crate::expr::{Expr, LiteralValue};
use crate::rlvalue::RlValue;
use crate::token::{Token, TokenType};
use crate::ErrorReporter;

pub struct Interpreter {
    _error_reporter: ErrorReporter,
}

impl Interpreter {
    pub fn new(error_reporter: ErrorReporter) -> Self {
        Self {
            _error_reporter: error_reporter,
        }
    }

    // TODO: rename me to something more reasonable
    pub fn visit(expr: &Expr) -> Result<RlValue> {
        use Expr::*;
        match expr {
            Assign(_t, _e) => Ok(RlValue::Nil),
            Binary(l, t, r) => {
                let left = Self::visit(l)?;
                let right = Self::visit(r)?;

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
            Grouping(e) => Self::visit(e.as_ref()),
            Literal(l) => Ok(RlValue::from(l)),
            Logical(_l, _t, _r) => Ok(RlValue::Nil),
            Set(_l, _t, _r) => Ok(RlValue::Nil),
            Super(_t1, _t2) => Ok(RlValue::Nil),
            This(_t) => Ok(RlValue::Nil),
            Unary(t, e) => {
                let right = Self::visit(e)?;
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
