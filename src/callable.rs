use anyhow::{anyhow, Result};

use crate::stmt::Stmt;
use crate::token::Token;
use crate::{interpreter::Interpreter, rlvalue::RlValue};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum Callable {
    BuiltIn(BuiltInFunction),
    Dynamic { params: Vec<Token>, body: Vec<Stmt> },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltInFunction {
    Clock,
}

impl TryFrom<Stmt> for Callable {
    type Error = anyhow::Error;
    fn try_from(s: Stmt) -> Result<Self, Self::Error> {
        match s {
            Stmt::Function { params, body, .. } => Ok(Callable::Dynamic { params, body }),
            _ => Err(anyhow!("wrong Stmt variant: {:?}", s)),
        }
    }
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
        _args: &Vec<RlValue>,
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

    pub fn call(&mut self, interpreter: &mut Interpreter, args: &Vec<RlValue>) -> Result<RlValue> {
        match self {
            Callable::BuiltIn(f) => Self::builtin_call(*f, interpreter, args),
            Callable::Dynamic { params, body } => {
                let env = interpreter.new_env_from_globals();

                for (i, param) in params.iter().enumerate() {
                    env.define(param.clone(), args.get(i).cloned());
                }

                interpreter.execute_block(&body, env)?;
                Ok(RlValue::Nil)
            }
        }
    }
}
