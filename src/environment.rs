use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::error::{Result, RloxError};
use crate::{callable::Callable, rlvalue::RlValue, token::Token};

/// A place to store level-scoped variables
pub struct Environment {
    // at least some form of interior mutability (yay!)
    values: RefCell<HashMap<String, Option<RlValue>>>,

    // a parent Environment. If it's None, it's the outer-most environment.
    enclosing: Option<Rc<RefCell<Environment>>>,

    // Temp identifier for debugging
    id: i32,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enc_id: i32 = match self.enclosing {
            Some(ref e) => e.borrow().id,
            None => -1,
        };
        write!(
            f,
            "Environment: values.len: {}, id: {}, enclosing_id: {}",
            self.values.borrow().len(),
            self.id,
            enc_id
        )
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let enc_id: i32 = match self.enclosing {
            Some(ref e) => e.borrow().id,
            None => -1,
        };
        f.debug_struct("Environment")
            .field("id", &self.id)
            .field("enclosing id", &enc_id)
            .finish()
    }
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>, id: i32) -> Self {
        Self {
            values: Default::default(),
            enclosing,
            id,
        }
    }

    pub fn define(&self, key: Token, value: Option<RlValue>) {
        self.values.borrow_mut().insert(key.lexeme, value);
    }

    pub fn get(&self, key: &Token) -> Result<Option<RlValue>> {
        if let Some(v) = self.values.borrow().get(&key.lexeme) {
            return Ok(v.clone());
        }

        if let Some(e) = &self.enclosing {
            return e.borrow().get(key);
        }

        // if we're at the outer-most Env and we still haven't found the symbol,
        // try looking into the "built-in functions" as defined in
        // Callable::BuiltInFunction. This is a bit of a hack, but works
        // for the current state (as of chapter 10 ...)
        match Callable::find_builtin(&key.lexeme) {
            Some(builtin) => Ok(Some(RlValue::Callable(builtin))),
            None => Err(RloxError::UndefinedSymbol(key.lexeme.clone())),
        }
    }

    pub fn get_at(&self, distance: u32, token: &Token) -> Result<RlValue> {
        if 0 == distance {
            return match self.values.borrow().get(&token.lexeme) {
                Some(v) => Ok(v.clone().expect("should have a defined rlvalue")),
                None => Err(RloxError::ResolutionError(format!(
                    "should have a defined rlvalue for token {:?}",
                    token,
                ))),
            };
        }

        match self.enclosing {
            Some(ref e) => Rc::clone(e).borrow().get_at(distance - 1, token),
            None => Err(RloxError::ResolutionError(String::from(
                "No more envs left to upwardly traverse?!?",
            ))),
        }
    }

    /// Looks up the key in the values map, but will not recurse up
    /// to the enclosing. Mainly for testing.
    #[allow(dead_code)]
    fn get_local(&self, key: &Token) -> Result<Option<RlValue>> {
        match self.values.borrow().get(&key.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(RloxError::UndefinedVariable(key.lexeme.clone())),
        }
    }

    pub fn assign(&self, key: &Token, value: RlValue) -> Result<()> {
        if self.values.borrow().contains_key(&key.lexeme) {
            self.values
                .borrow_mut()
                .entry(key.lexeme.clone())
                .and_modify(|e| *e = Some(value));
            return Ok(());
        }

        match &self.enclosing {
            Some(enclosing) => enclosing.borrow().assign(key, value),
            None => Err(RloxError::UndefinedVariable(key.lexeme.clone())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Result;
    use crate::token::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn simple_read() -> Result<()> {
        let env = Rc::new(RefCell::new(Environment::new(None, 0)));

        let key = Token::simple_token(TokenType::Var, String::from("k1"), 1);
        let value = RlValue::Boolean(true);

        env.borrow().define(key.clone(), Some(value));
        let res = env.borrow().get(&key)?;
        assert!(matches!(res, Some(RlValue::Boolean(true))));

        Ok(())
    }

    #[test]
    fn nested_read() -> Result<()> {
        let outer = Rc::new(RefCell::new(Environment::new(None, 0)));
        let outer_cpy = Rc::clone(&outer);
        let inner = Rc::new(RefCell::new(Environment::new(Some(outer_cpy), 1)));

        let key = Token::simple_token(TokenType::Var, String::from("k1"), 1);
        let value = RlValue::Boolean(true);

        outer.borrow().define(key.clone(), Some(value));
        let res = outer.borrow().get(&key)?;
        assert!(matches!(res, Some(RlValue::Boolean(true))));

        let res = inner.borrow().get(&key)?;
        assert!(matches!(res, Some(RlValue::Boolean(true))));

        let res = inner.borrow().get_local(&key);
        assert!(res.is_err());

        Ok(())
    }
}
