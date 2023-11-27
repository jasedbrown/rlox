use anyhow::{anyhow, Result};

use crate::{interpreter::Interpreter, rlvalue::RlValue};
use std::time::{SystemTime, UNIX_EPOCH};

// pub trait Callable {
//     fn arity(&self) -> usize;

//     fn call(&self, interpreter: &Interpreter, args: &[RlValue]) -> Result<RlValue>;
// }

pub enum Callable {
    BuiltIn(BuiltInFunction),
    Dynamic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltInFunction {
    Clock,
}

impl Callable {
    // TODO: not sure if this is better as a From which returns Option<Callable>.
    // tryFrom returns a Result<>, but an Err isn't quite right for not finding a builtin.
    pub fn find_builtin(name: &str) -> Option<Callable> {
        match name {
            "clock" => Some(Callable::BuiltIn(BuiltInFunction::Clock)),
            _ => None,
        }
    }

    fn builtin_arity(f: BuiltInFunction) -> usize {
        use BuiltInFunction::*;

        match f {
            Clock => 0,
        }
    }

    fn builtin_call(
        f: BuiltInFunction,
        _interpreter: &Interpreter,
        _args: &[RlValue],
    ) -> Result<RlValue> {
        use BuiltInFunction::*;

        match f {
            Clock => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                Ok(RlValue::Double(now as f64))
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Callable::BuiltIn(f) => Self::builtin_arity(*f),
            Callable::Dynamic => 0,
        }
    }

    pub fn call(&self, interpreter: &Interpreter, args: &[RlValue]) -> Result<RlValue> {
        match self {
            Callable::BuiltIn(f) => Self::builtin_call(*f, interpreter, args),
            Callable::Dynamic => Err(anyhow!("Not implemented yet!")),
        }
    }
}
