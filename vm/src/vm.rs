use crate::chunk::{Chunk, Opcode, Value};
use anyhow::{anyhow, Context, Result};

pub(crate) struct VM {
  chunk: Chunk,
  stack: Vec<Value>,
}

impl VM {
  pub(crate) fn new(chunk: Chunk) -> Self {
    VM {
      stack: vec![],
      chunk,
    }
  }

  pub(crate) fn interpret(&mut self) -> Result<()> {
    macro_rules! pop_stack {
      () => {
        self.stack.pop().context("empty stack")?
      };
    }

    // TODO: make `Chunk` an iterator
    for opcode in self.chunk.code.iter() {
      match opcode {
        Opcode::Return => {
          println!("{:?}", self.stack.pop());
        }
        Opcode::Constant {
          index: constant_index,
        } => {
          self
            .stack
            .push(self.chunk.get_constant(*constant_index).clone());
        }
        Opcode::Negate => {
          let value = self.stack.last_mut().unwrap();

          if let Value::Number(n) = value {
            *n = -*n;
          } else {
            return Err(anyhow!("only numbers can be negated"));
          }
        }
        Opcode::Multiply | Opcode::Subtract | Opcode::Divide | Opcode::Less | Opcode::Greater => {
          let Value::Number(b) = pop_stack!() else {
            return Err(anyhow!("expected a number"));
          };
          let Value::Number(a) = pop_stack!() else {
            return Err(anyhow!("expected a number"));
          };

          let result = match opcode {
            Opcode::Subtract => Value::Number(a - b),
            Opcode::Multiply => Value::Number(a * b),
            Opcode::Divide => Value::Number(a / b),
            Opcode::Less => Value::Bool(a < b),
            Opcode::Greater => Value::Bool(a > b),
            _ => panic!("Will not happen."),
          };

          self.stack.push(result);
        }
        Opcode::Add => {
          let b = pop_stack!();
          let a = pop_stack!();

          self.stack.push(
            if matches!(a, Value::String(_)) || matches!(b, Value::String(_)) {
              Value::String(format!("{}{}", a, b))
            } else {
              let Value::Number(b) = b else {
                return Err(anyhow!("expected a number"));
              };
              let Value::Number(a) = a else {
                return Err(anyhow!("expected a number"));
              };

              Value::Number(a + b)
            },
          );
        }
        Opcode::Equal => {
          let a = pop_stack!();
          let b = pop_stack!();

          self.stack.push(Value::Bool(a.is_truthy() == b.is_truthy()));
        }
        Opcode::Not => {
          let v = pop_stack!().is_truthy();

          self.stack.push(Value::Bool(!v));
        }
        Opcode::True => {
          self.stack.push(Value::Bool(true));
        }
        Opcode::False => {
          self.stack.push(Value::Bool(false));
        }
        Opcode::Nil => {
          self.stack.push(Value::Nil);
        }
      }
    }

    println!("Result: {:?}", self.stack);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_interpret() {
    let mut chunk = Chunk::new();

    chunk.push_constant(Value::Number(1.), 1);
    chunk.push_constant(Value::Number(2.), 1);
    chunk.push_code(Opcode::Add, 1);
    chunk.push_code(Opcode::Negate, 1);
    chunk.push_code(Opcode::Return, 1);

    let mut vm = VM::new(chunk);

    vm.interpret().unwrap();
  }
}
