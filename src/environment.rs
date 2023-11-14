use anyhow::{anyhow, Result};
use std::collections::{hash_map::Entry, HashMap};

use crate::{rlvalue::RlValue, token::Token};

#[derive(Debug, Default)]
pub struct Environment {
    // TODO interior mutability??
    values: HashMap<String, Option<RlValue>>,
}

impl Environment {
    pub fn define(&mut self, key: String, value: Option<RlValue>) {
        self.values.insert(key, value);
    }

    pub fn get(&mut self, key: &Token) -> Result<Option<RlValue>> {
        match self.values.entry(key.lexeme.clone()) {
            Entry::Occupied(e) => Ok(e.get().clone()),
            Entry::Vacant(_) => Err(anyhow!("Undefined variable: {:?}", &key.lexeme)),
        }
    }
}
