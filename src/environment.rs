use anyhow::{anyhow, Result};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::{rlvalue::RlValue, token::Token};

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

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>, id: i32) -> Self {
        let s = Self {
            values: Default::default(),
            enclosing,
            id,
        };

        println!("Env::new HEAD ({}) ", s);
        s
    }

    pub fn define(&self, key: Token, value: Option<RlValue>) {
        println!("Env::define HEAD ({})  - key: {:?}", self, &key);
        self.values.borrow_mut().insert(key.lexeme, value);
    }

    pub fn get(&self, key: &Token) -> Result<Option<RlValue>> {
        println!("Env::get HEAD ({})  - key: {:?}", self, &key);

        if let Some(v) = self.values.borrow().get(&key.lexeme) {
            return Ok(v.clone());
        }
        match &self.enclosing {
            Some(e) => return e.borrow().get(key),
            None => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
        }
    }

    /// Looks up the key in the values map, but will not recurse up
    /// to the enclosing. Mainly for testing.
    #[allow(dead_code)]
    fn get_local(&self, key: &Token) -> Result<Option<RlValue>> {
        println!("Env::get_local HEAD ({})  - key: {:?}", self, &key);

        match self.values.borrow().get(&key.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
        }
    }

    pub fn assign(&self, key: &Token, value: RlValue) -> Result<()> {
        println!("Env::assign HEAD ({})  - key: {:?}", self, &key);
        if self.values.borrow().contains_key(&key.lexeme) {
            self.values
                .borrow_mut()
                .entry(key.lexeme.clone())
                .and_modify(|e| *e = Some(value));
            return Ok(());
        }

        println!("Env::assign MID ({})  - key: {:?}", self, &key);
        let x = match &self.enclosing {
            Some(enclosing) => enclosing.borrow().assign(key, value),
            None => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
        };
        println!("Env::assign  ({})  - key: {:?}", self, &key);
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::*;
    use anyhow::Result;
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
