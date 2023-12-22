use std::fmt;
use std::fmt::Write;

pub(crate) enum Opcode {
  Return,
  Constant { index: usize },
  Negate,
  Add,
  Multiply,
  // Subtract,
  // Divide
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
  Number(f64),
  String(String)
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
          Opcode::Negate => {
            write!(&mut buf, " {: <15}", "NEGATE").unwrap();
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
