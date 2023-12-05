use crate::interpreter::Value;
use std::cell::RefCell;
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

  fn execute_at_mut<T>(&mut self, distance: usize, fun: impl Fn(&mut Self) -> T) -> T {
    if distance == 0 {
      fun(self)
    } else {
      let Some(parent) = &self.parent else {
        panic!("cant find a parent env")
      };

      parent.borrow_mut().execute_at_mut(distance - 1, fun)
    }
  }

  fn execute_at<T>(&self, distance: usize, fun: impl Fn(&Self) -> T) -> T {
    if distance == 0 {
      fun(self)
    } else {
      let Some(parent) = &self.parent else {
        panic!("cant find a parent env")
      };
      parent.borrow().execute_at(distance - 1, fun)
    }
  }

  pub(crate) fn define(&mut self, identifier: &str, value: Rc<Value>) {
    self.values.insert(identifier.to_string(), value);
  }

  pub(crate) fn assign(
    &mut self,
    identifier: &str,
    value: Rc<Value>,
    distance: usize,
  ) -> Rc<Value> {
    self.execute_at_mut(distance, |env| {
      env.values.insert(identifier.to_string(), Rc::clone(&value));

      value.clone()
    })
  }

  pub(crate) fn get(&self, identifier: &str, distance: usize) -> Option<Rc<Value>> {
    self.execute_at(distance, |env| env.values.get(identifier).map(Rc::clone))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_define() {}
}
