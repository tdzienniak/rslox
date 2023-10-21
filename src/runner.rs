use crate::scanner::Scanner;
use anyhow::Result;

pub fn run(source: String) -> Result<()> {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens()?;

  for token in tokens {
    println!("{:?}", token);
  }

  Ok(())
}
