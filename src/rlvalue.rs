#![allow(dead_code)]
use std::fmt;

use crate::callable::Callable;
use crate::expr::LiteralValue;

/// The result value and type of evaluating an expression.
/// This first attempt is a basic enum tag [0], but
/// it'd be nifty to build a nan box for the next generation.
///
/// [0] https://piotrduperas.com/posts/nan-boxing
#[derive(Clone, Debug, Default)]
pub enum RlValue {
    #[default]
    Nil,
    Boolean(bool),
    Double(f64),
    String(String),
    Callable(Callable),
}

impl fmt::Display for RlValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RlValue::Nil => write!(f, "nil"),
            RlValue::Boolean(b) => write!(f, "{}", b),
            RlValue::Double(d) => write!(f, "{:2}", d),
            RlValue::String(ref s) => write!(f, "{}", s),
            // TODO: something better than this!
            RlValue::Callable(ref c) => write!(f, "this is a callable: {:?}", c),
        }
    }
}

impl RlValue {
    pub fn is_nil(&self) -> bool {
        matches!(self, RlValue::Nil)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, RlValue::Boolean(_))
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, RlValue::Double(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, RlValue::String(_))
    }

    pub fn is_truthy(&self) -> bool {
        match *self {
            RlValue::Nil => false,
            RlValue::Boolean(b) => b,
            _ => true,
        }
    }

    pub fn as_numeric(&self) -> Option<f64> {
        match *self {
            RlValue::Double(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match *self {
            RlValue::String(ref r) => Some(r.to_string()),
            _ => None,
        }
    }
}

impl From<LiteralValue> for RlValue {
    fn from(l: LiteralValue) -> Self {
        match l {
            LiteralValue::Nil() => RlValue::Nil,
            LiteralValue::Boolean(b) => RlValue::Boolean(b),
            LiteralValue::Number(d) => RlValue::Double(d),
            // TODO: can probably do something better than clone(), but it
            // works for this PL exercise.
            LiteralValue::String(ref s) => RlValue::String(s.clone()),
        }
    }
}

impl From<&LiteralValue> for RlValue {
    fn from(l: &LiteralValue) -> Self {
        match *l {
            LiteralValue::Nil() => RlValue::Nil,
            LiteralValue::Boolean(b) => RlValue::Boolean(b),
            LiteralValue::Number(d) => RlValue::Double(d),
            // TODO: can probably do something better than clone(), but it
            // works for this PL exercise.
            LiteralValue::String(ref s) => RlValue::String(s.clone()),
        }
    }
}
