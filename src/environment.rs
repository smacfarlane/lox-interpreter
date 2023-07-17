use crate::data_types::Object;
use crate::error::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclose(enclosed: &Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosed.clone())), // TODO: Does this need to be a reference to the parent?
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
        &self;
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }
        dbg!(&self);
        if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(anyhow!("undefined variable '{}'", name))
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object> {
        let token = token.clone();
        let name = token
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(token.clone()))?;

        let value = self.values.get(&name);

        match (value, &self.enclosing) {
            (Some(value), _) => Ok(value.clone()),
            (None, Some(ref enclosing)) => enclosing.get(&token),
            (None, None) => Err(RuntimeError::UndefinedVariable(name.clone()).into()),
        }
    }
}
