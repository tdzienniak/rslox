use std::fmt;
use std::fmt::{Display, Write};

#[derive(Clone)]
pub(crate) enum Opcode {
  Return,
  Constant { index: usize },
  Not,
  True,
  False,
  Nil,
  Equal,
  Greater,
  Less,
  Negate,
  Add,
  Multiply,
  Subtract,
  Divide,
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
  Number(f64),
  String(String),
  Bool(bool),
  Nil,
}

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Value::Number(v) => v.to_string(),
        Value::String(v) => v.to_string(),
        Value::Nil => "nil".to_string(),
        Value::Bool(v) => v.to_string(),
      }
    )
  }
}

impl Value {
  pub(crate) fn is_truthy(&self) -> bool {
    match self {
      Value::Nil => false,
      Value::Bool(v) => *v,
      _ => true,
    }
  }
}

pub(crate) struct Chunk {
  constants: Vec<Value>,
  pub(crate) code: Vec<Opcode>,
  lines: Vec<u32>,
}

impl Chunk {
  pub(crate) fn new() -> Self {
    Chunk {
      code: vec![],
      constants: vec![],
      lines: vec![],
    }
  }

  pub(crate) fn push_constant(&mut self, value: Value, line: u32) {
    self.constants.push(value);

    let constant_index = self.constants.len() - 1;

    self.push_code(
      Opcode::Constant {
        index: constant_index,
      },
      line,
    );
  }

  pub(crate) fn get_constant(&self, index: usize) -> &Value {
    &self.constants[index]
  }

  pub(crate) fn push_code(&mut self, code: Opcode, line: u32) {
    self.code.push(code);
    self.lines.push(line);
  }
}

impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result = self
      .code
      .iter()
      .enumerate()
      .map(|(index, opcode)| {
        let mut buf = String::new();
        write!(&mut buf, "{:0>4}", index).unwrap();

        if index > 0 && self.lines[index] == self.lines[index - 1] {
          write!(&mut buf, "{: >5}", "|").unwrap();
        } else {
          write!(&mut buf, "{: >5}", self.lines[index]).unwrap();
        }

        match opcode {
          Opcode::Return => {
            write!(&mut buf, " {: <15}", "RETURN").unwrap();
          }
          Opcode::Constant {
            index: constant_index,
          } => {
            write!(
              &mut buf,
              " {: <15}{:0>3}: {:?}",
              "CONSTANT", constant_index, self.constants[*constant_index]
            )
            .unwrap();
          }
          Opcode::Add => {
            write!(&mut buf, " {: <15}", "ADD").unwrap();
          }
          Opcode::Multiply => {
            write!(&mut buf, " {: <15}", "MULT").unwrap();
          }
          Opcode::Subtract => {
            write!(&mut buf, " {: <15}", "SUB").unwrap();
          }
          Opcode::Divide => {
            write!(&mut buf, " {: <15}", "DIV").unwrap();
          }
          Opcode::Negate => {
            write!(&mut buf, " {: <15}", "NEGATE").unwrap();
          }
          Opcode::Not => {
            write!(&mut buf, " {: <15}", "NOT").unwrap();
          }
          Opcode::True => {
            write!(&mut buf, " {: <15}", "TRUE").unwrap();
          }
          Opcode::False => {
            write!(&mut buf, " {: <15}", "FALSE").unwrap();
          }
          Opcode::Nil => {
            write!(&mut buf, " {: <15}", "NIL").unwrap();
          }
          Opcode::Equal => {
            write!(&mut buf, " {: <15}", "EQUAL").unwrap();
          }
          Opcode::Less => {
            write!(&mut buf, " {: <15}", "LESS").unwrap();
          }
          Opcode::Greater => {
            write!(&mut buf, " {: <15}", "GREATER").unwrap();
          }
        };

        buf
      })
      .collect::<Vec<String>>()
      .join("\n");

    write!(f, "{}", result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_display() {
    let mut chunk = Chunk::new();

    chunk.push_code(Opcode::Return, 1);
    chunk.push_code(Opcode::Return, 1234);
    chunk.push_constant(Value::Number(3.14), 2);

    print!("{}", chunk);
  }
}
