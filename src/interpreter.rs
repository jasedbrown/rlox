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

                match *t {
                    TokenType::Minus => {
                        let left_d = left.as_numeric().unwrap();
                        let right_d = right.as_numeric().unwrap();
                        let ret_val: f64 = (left_d - right_d) as f64;
                        return Ok(RlValue::Double(ret_val));
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

    fn evaluate(expr: &Expr) -> Result<RlValue> {
        // asdfasdf
        //        expr.accept() // ???????????
        Ok(RlValue::Double(42.))
    }
}
