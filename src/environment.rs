use anyhow::{anyhow, Result};

use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};
use std::rc::Rc;

use crate::{rlvalue::RlValue, token::Token};

/// A place to store level-scoped variables
pub struct Environment {
    // at least some form of interior mutability (yay!)
    values: RefCell<HashMap<String, Option<RlValue>>>,

    // a parent Environment. If it's None, it's the outer-most environment.
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: Default::default(),
            enclosing,
        }
    }

    pub fn define(&self, key: String, value: Option<RlValue>) {
        self.values.borrow_mut().insert(key, value);
    }

    pub fn get(&self, key: &Token) -> Result<Option<RlValue>> {
        // TODO: there's a better way to express in a function style ...
        match self.values.borrow_mut().entry(key.lexeme.clone()) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(_) => match &self.enclosing {
                Some(enclosing) => Ok(enclosing.borrow().get(key)?),
                None => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
            },
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

        if let Some(enclosing) = &self.enclosing {
            return Ok(enclosing.borrow().assign(key, value)?);
        }

        Err(anyhow!("Undefined variable: {:?}", &key.lexeme))
    }
}
