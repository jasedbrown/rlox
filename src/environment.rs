use anyhow::{anyhow, Result};

use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap};

use crate::{rlvalue::RlValue, token::Token};

/// A place to store level-scoped variables
#[derive(Default)]
pub struct Environment {
    // TODO(jeb): Not sure if RefCell is the best here, but it's
    // at least some form of interior mutability (yay!)
    values: RefCell<HashMap<String, Option<RlValue>>>,
}

impl Environment {
    pub fn define(&self, key: String, value: Option<RlValue>) {
        self.values.borrow_mut().insert(key, value);
    }

    pub fn get(&self, key: &Token) -> Result<Option<RlValue>> {
        match self.values.borrow_mut().entry(key.lexeme.clone()) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(_) => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
        }
    }
}
