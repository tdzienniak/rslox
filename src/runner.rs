use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Result;

pub fn run(source: String) -> Result<()> {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens()?;
  let mut parser = Parser::new(tokens);

  Ok(())
}
