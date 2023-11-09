use crate::environment::Environment;
use crate::interpreter::Interpret;
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Result;

pub fn run(source: String) -> Result<()> {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens()?;
  let mut parser = Parser::new(tokens);

  let statements = parser.parse()?;
  let mut environment = Environment::new(None);

  for stmt in &statements {
    stmt.interpret(&mut environment)?;
  }

  Ok(())
}
