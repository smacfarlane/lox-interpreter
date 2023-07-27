use crate::data_types::Object;
use crate::error::RuntimeError;
use crate::token::Token;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
struct Scope(HashMap<String, Object>);

impl std::ops::Deref for Scope {
    type Target = HashMap<String, Object>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Scope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![Scope(HashMap::new())],
        }
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(Scope(HashMap::new()));
    }

    // TODO: Does this need to be a result?
    pub fn end_scope(&mut self) -> Result<()> {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            Ok(())
        } else {
            Err(anyhow!("cannot end global scope"))
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        // Safety: end_scope does not pop the last element
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return Ok(());
            }
        }

        Err(anyhow!("undefined variable '{}'", name))
    }

    pub fn get(&self, token: &Token) -> Result<Object> {
        let token = token.clone();
        let name = token
            .lexeme
            .clone()
            .ok_or(RuntimeError::UnexpectedToken(token.clone()))?;

        dbg!(&self.scopes);
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(&name) {
                return Ok(scope.get(&name).unwrap().to_owned());
            }
        }

        Err(RuntimeError::UndefinedVariable(name.clone()).into())
    }
}
