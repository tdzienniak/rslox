use crate::environment::Environment;
use crate::interpreter::{Interpret, NativeClock, NativePrint, Value};
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;


pub fn run(source: String) -> Result<()> {
  let scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens()?;
  let mut parser = Parser::new(tokens);

  let statements = parser.parse()?;
  let environment = Rc::new(RefCell::new(Environment::new(None)));

  {
    let mut env = environment
      .borrow_mut();

    env.define("clock", Rc::new(Value::Function(Box::new(NativeClock {}))));
    env.define("println", Rc::new(Value::Function(Box::new(NativePrint {}))));
  }

  for stmt in &statements {
    stmt.interpret(Rc::clone(&environment))?;
  }

  Ok(())
}
