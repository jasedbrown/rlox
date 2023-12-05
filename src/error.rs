#![allow(dead_code)]

use thiserror::Error;

use crate::rlvalue::RlValue;

/// An `RloxError` wrapper for both garden variety errors
/// as well as early-return (and just any return) from a
/// Rlox `Callable`.
///
/// I originally had this as two separate types, with a parent enum,
/// but the extra indirection is a bit much for this side project ....
#[derive(Debug, Error)]
pub enum RloxError {
    #[error("not-an-error, just return this value ...")]
    Return(Option<RlValue>),

    #[error("expected {0} args, but got {1}")]
    ArityError(usize, usize),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    ParseError(String),

    #[error("{0}")]
    UndefinedSymbol(String),

    #[error("{0}")]
    UndefinedVariable(String),

    #[error("{0}")]
    IncorrectType(String),

    #[error("{0}")]
    ResolveError(String),

    #[error("how did you get here?!?!: {0}")]
    Unreachable(String),

    #[error("{0}")]
    Unsupported(String),
}

pub type Result<T, E = RloxError> = core::result::Result<T, E>;
