use crate::data_types::Object;
use crate::error::RuntimeError;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tracing::instrument;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope(HashMap<String, Object>);

impl Scope {
    pub fn new() -> Scope {
        Scope(HashMap::new())
    }
}

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

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![Scope::new()],
        }
    }

    pub fn contains(e: &Environment) -> Self {
        let mut env = Environment {
            scopes: e.scopes.clone(),
        };
        env.new_scope();
        env
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }

    #[instrument(skip(self))]
    pub fn new_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    // TODO: Does this need to be a result?
    #[instrument(skip(self))]
    pub fn end_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    #[instrument(skip(self))]
    pub fn define(&mut self, name: String, value: Object) {
        match self.scopes.last_mut() {
            Some(scope) => scope.insert(name, value),
            None => unreachable!(),
        };
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

    #[instrument(skip(self))]
    pub fn assign_at(&mut self, distance: usize, name: String, value: Object) -> Result<()> {
        let distance = self.scopes.len() - (distance + 1);
        match self.scopes.get_mut(distance) {
            Some(scope) => {
                scope.insert(name, value);
                Ok(())
            }
            None => Err(anyhow!("exeeded scope depth: {}", distance)),
        }
    }

    #[instrument(skip(self))]
    pub fn assign_global(&mut self, name: String, value: Object) -> Result<()> {
        if self.scopes[0].contains_key(&name) {
            self.scopes[0].insert(name, value);
            Ok(())
        } else {
            Err(anyhow!("undefined variable '{}'", name))
        }
    }

    #[instrument(skip(self))]
    pub fn get(&self, name: &str) -> Result<Object> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return Ok(scope.get(name).unwrap().to_owned());
            }
        }
        Err(RuntimeError::UndefinedVariable(name.to_string()).into())
    }

    pub fn get_global(&self, name: &str) -> Result<Object> {
        if self.scopes[0].contains_key(name) {
            Ok(self.scopes[0].get(name).unwrap().to_owned())
        } else {
            Err(RuntimeError::UndefinedVariable(name.to_string()).into())
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Object> {
        let mut iter = self.scopes.iter().rev();

        for _ in 0..distance {
            iter.next();
        }

        // match self.scopes.get(distance) {
        match iter.next() {
            Some(scope) => scope
                .get(name)
                .cloned()
                .ok_or(RuntimeError::UndefinedVariable(name.to_string()).into()),
            None => Err(anyhow!(
                "exceeded scope depth: {} > {:?}",
                distance,
                self.scopes
            )),
        }
    }
}
