use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::Value;

pub(crate) struct Environment {
  values: HashMap<String, Rc<Value>>,
  parent: Option<Box<Environment>>,
}

impl Environment {
  pub(crate) fn new(parent: Option<Environment>) -> Self {
    Environment {
      values: HashMap::new(),
      parent: None
    }
  }

  pub(crate) fn define(&mut self, identifier: &str, value: Rc<Value>) {
    self.values.insert(identifier.to_string(), value);
  }

  pub(crate) fn get(&self, identifier: &str) -> Option<Rc<Value>> {
    self.values.get(identifier).cloned()
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {

    }
}
