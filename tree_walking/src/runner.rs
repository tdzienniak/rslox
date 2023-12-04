use crate::interpreter::{Interpreter};
use crate::parser::Parser;
use crate::scanner::{Scanner, Token};
use anyhow::Result;
use crate::resolver::Resolver;

pub fn run(source: String) -> Result<()> {
  let scanner = Scanner::new(source);

  let tokens = scanner.collect::<Result<Vec<Token>>>()?;

  let mut parser = Parser::new(tokens);

  let statements = parser.parse()?;
  let resolver = Resolver::new();

  let locals = resolver.resolve_program(&statements);
  println!("{:?}", locals);

  let interpreter = Interpreter::new(locals);

  interpreter.interpret_program(statements);

  Ok(())
}
