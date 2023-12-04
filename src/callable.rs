use crate::environment::Environment;
use crate::error::Result;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::{interpreter::Interpreter, rlvalue::RlValue};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum Callable {
    BuiltIn(BuiltInFunction),
    Dynamic {
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Callable::BuiltIn(func) => write!(f, "{:?}", func),
            Callable::Dynamic { params, .. } => write!(f, "function with arity {}", params.len()),
        }
    }
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
            Callable::Dynamic { params, .. } => params.len(),
        }
    }

    pub fn call(&mut self, interpreter: &mut Interpreter, args: &[RlValue]) -> Result<RlValue> {
        match self {
            Callable::BuiltIn(f) => Self::builtin_call(*f, interpreter, args),
            Callable::Dynamic {
                params,
                body,
                closure,
            } => {
                let c = Rc::clone(closure);
                let env = Environment::new(Some(c), 42);

                for (i, param) in params.iter().enumerate() {
                    env.define(param.clone(), args.get(i).cloned());
                }

                Ok(interpreter.execute_block(body, env)?)
            }
        }
    }
}
