use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::interpreter::Interpret;
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Result;

pub fn run(source: String) -> Result<()> {
  let scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens()?;
  let mut parser = Parser::new(tokens);

  let statements = parser.parse()?;
  let environment = Rc::new(RefCell::new(Environment::new(None)));

  for stmt in &statements {
    stmt.interpret(Rc::clone(&environment))?;
  }

  Ok(())
}
