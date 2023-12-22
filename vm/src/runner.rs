use crate::parser::Parser;
use anyhow::Result;
use scanner::Scanner;
use crate::vm::VM;

pub fn run(source: String) -> Result<()> {
  let scanner = Scanner::new(source);

  let mut parser = Parser::new(scanner);

  parser.parse()?;

  let chunk = parser.take_chunk();

  println!("{}\n", chunk);

  let mut vm = VM::new(chunk);

  vm.interpret()?;

  Ok(())
}
