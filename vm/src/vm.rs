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
    // TODO: make `Chunk` an iterator
    for (index, opcode) in self.chunk.code.iter().enumerate() {
      match opcode {
        Opcode::Return => {
          println!("{:?}", self.stack.pop());
        }
        Opcode::Constant {
          index: constant_index,
        } => {
          self.stack.push(self.chunk.get_constant(*constant_index).clone());
        }
        Opcode::Negate => {
          let value = self.stack.last_mut().unwrap();

          if let Value::Number(n) = value {
            *n = -*n;
          } else {
            return Err(anyhow!("only numbers can be negated"));
          }
        }
        Opcode::Add | Opcode::Multiply => {
          let Value::Number(b) = self.stack.pop().context("empty stack")? else {
            return Err(anyhow!("expected a number"));
          };
          let Value::Number(a) = self.stack.pop().context("empty stack")? else {
            return Err(anyhow!("expected a number"));
          };

          let result = match opcode {
            Opcode::Add => a + b,
            Opcode::Multiply => a * b,
            _ => panic!("Will not happen.")
          };

          self.stack.push(Value::Number(result));
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

    vm.interpret();
  }
}
