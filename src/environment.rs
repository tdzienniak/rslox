use std::cell::RefCell;
use crate::errors::RuntimeError;
use crate::interpreter::Value;
use anyhow::Result;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) struct Environment {
  values: HashMap<String, Rc<Value>>,
  parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub(crate) fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
    Environment {
      values: HashMap::new(),
      parent,
    }
  }

  pub(crate) fn define(&mut self, identifier: &str, value: Rc<Value>) {
    self.values.insert(identifier.to_string(), value);
  }

  pub(crate) fn get(&self, identifier: &str) -> Option<Rc<Value>> {
    if let Some(value) = self.values.get(identifier) {
      Some(Rc::clone(value))
    } else if let Some(parent) = &self.parent {
      parent.borrow().get(identifier)
    } else {
      None
    }
  }

  pub(crate) fn assign(&mut self, identifier: &str, value: Rc<Value>) -> Result<Rc<Value>> {
    if !self.values.contains_key(identifier) {
      Err(
        RuntimeError::AssignmentToUndeclaredVariable {
          identifier: identifier.to_string(),
        }
        .into(),
      )
    } else {
      self.values.insert(identifier.to_string(), value.clone());
      Ok(value)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_define() {}
}
