use thiserror::Error;

use crate::rlvalue::RlValue;

/// An `Error` wrapper for both garden variety errors
/// as well as early-return (and just any return) from a
/// Rlox `Callable`.
#[derive(Debug, Error)]
pub enum RloxReturnable {
    #[error(transparent)]
    Error(#[from] RloxError),

    #[error("not-an-error, just return this value ...")]
    Return(Option<RlValue>),
}

#[derive(Debug, Error)]
pub enum RloxError {
    #[error("expected {1} args, but got {1}")]
    ArityError(usize, usize),

    #[error("type is not allowed: {0}")]
    DisallowedType(String),

    #[error("{0}")]
    UndefinedSymbol(String),

    #[error("{0}")]
    UndefinedVariable(String),

    #[error("{0}")]
    IncorrectType(String),

    #[error("how did you get here?!?!: {0}")]
    Unreachable(String),

    #[error("{0}")]
    Unsupported(String),
}

pub type Result<T, E = RloxReturnable> = core::result::Result<T, E>;
