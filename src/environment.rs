use crate::data_types::Object;
use crate::error::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
        &self;
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            Err(anyhow!("undefined variable '{}'", name))
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        let name = name.clone();
        dbg!(self);
        if let Some(name) = name.lexeme {
            return self
                .values
                .get(&name)
                .cloned()
                .ok_or(RuntimeError::UndefinedVariable(name).into());
        }

        // What do we call a token that has no lexeme
        Err(RuntimeError::UnexpectedToken(name.clone()).into())
    }
}
