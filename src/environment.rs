use anyhow::{anyhow, Result};

use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};

use crate::{rlvalue::RlValue, token::Token};

/// A place to store level-scoped variables
pub struct Environment<'a> {
    // at least some form of interior mutability (yay!)
    values: RefCell<HashMap<String, Option<RlValue>>>,

    // a parent Environment. If it's None, it's the outer-most environment.
    // TODO: this is the lazy way for lifetimes ... needs love
    enclosing: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(enclosing: Option<&'a Environment>) -> Self {
        let enc = match enclosing {
            Some(e) => Some(e),
            None => None,
        };

        Self {
            values: Default::default(),
            enclosing: enc,
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
                Some(enclosing) => Ok(enclosing.get(key)?),
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
            return Ok(enclosing.assign(key, value)?);
        }

        Err(anyhow!("Undefined variable: {:?}", &key.lexeme))
    }
}
