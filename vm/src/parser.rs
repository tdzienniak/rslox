use anyhow::Result;
use scanner::{Scanner, Token, TokenType};
use thiserror::Error;

use crate::chunk::{Chunk, Value};

#[derive(Error, Debug, Clone)]
pub(crate) enum SyntaxError {
  #[error("';' expected at the end of a statement")]
  MissingSemicolon,
}

const NONE_PREC: u16 = 0;
const ASSIGNMENT_PREC: u16 = 1;
const TERM_PREC: u16 = 2;
const FACTOR_PREC: u16 = 3;
const UNARY_PREC: u16 = 4;

pub(crate) struct Parser {
  scanner: Scanner,
  previous: Option<Token>,
  current: Option<Token>,
  errors: Vec<SyntaxError>,
  chunk: Chunk
}

impl Parser {
  pub(crate) fn new(scanner: Scanner) -> Self {
    Self {
      scanner,
      current: None,
      previous: None,
      errors: vec![],
      chunk: Chunk::new()
    }
  }

  pub(crate) fn parse(&mut self) -> Result<()> {
    self.advance()?;
    self.expression()?;

    Ok(())
  }

  pub(crate) fn take_chunk(self) -> Chunk {
    self.chunk
  }

  fn get_precedence(&self, token_type: &TokenType) -> u16 {
    match token_type {
      TokenType::Plus => TERM_PREC,
      TokenType::Minus => TERM_PREC,
      TokenType::Star => FACTOR_PREC,
      TokenType::Slash => FACTOR_PREC,
      _ => NONE_PREC,
    }
  }

  fn parse_prefix(&mut self) -> Result<()> {
    let token = self.previous();
    match token.kind {
      TokenType::Number(value) => {
        self.chunk.push_constant(Value::Number(value), token.line);
      },
      TokenType::Minus => {
        self.parse_unary()?;
      },
      TokenType::LeftParen => {
        self.expression()?;
        self.consume(TokenType::RightParen, SyntaxError::MissingSemicolon)?;
      },
      _ => panic!("Unexpected token for prefix: {:?}", token)
    };

    Ok(())
  }

  fn parse_infix(&mut self) -> Result<()> {
    let operator_token = self.previous().clone();

    match operator_token.kind {
      TokenType::Plus | TokenType::Star => {
        // parse right
        // TODO: support left and right associativity
        self.parse_precedence(self.get_precedence(&operator_token.kind) + 1)?;

        match operator_token.kind {
          TokenType::Plus => {
            self.chunk.push_code(crate::chunk::Opcode::Add, operator_token.line)
          },
          TokenType::Star => {
            self.chunk.push_code(crate::chunk::Opcode::Multiply, operator_token.line)
          },
          _ => panic!("This will not happen, but compiler needs to be happpy.")
        }
      }
      _ => panic!("Unexpected token for infix operator"),
    };

    Ok(())
  }

  pub(crate) fn parse_precedence(&mut self, prec: u16) -> Result<()> {
    self.advance()?;

    self.parse_prefix()?;

    // parse infix
    while prec <= self.get_precedence(&self.current().kind) {
      self.advance()?;

      self.parse_infix()?;
    }

    Ok(())
  }

  pub(crate) fn expression(&mut self) -> Result<()> {
    self.parse_precedence(ASSIGNMENT_PREC)
  }

  fn parse_unary(&mut self) -> Result<()> {
    let operator_token = self.previous().clone();
    self.parse_precedence(UNARY_PREC)?;

    match operator_token.kind {
      TokenType::Bang => {
        println!("PREFIX BANG")
      }
      TokenType::Minus => {
        self.chunk.push_code(crate::chunk::Opcode::Negate, operator_token.line)
      }
      _ => {
        panic!("Token {:?} is not a prefix operator.", operator_token);
      }
    }

    Ok(())
  }
  
  fn consume(&mut self, token_type: TokenType, err: SyntaxError) -> Result<()> {
    if self.current().kind == token_type {
      self.advance()
    } else {
      Err(err.into())
    }
  }
  
  fn advance(&mut self) -> Result<()> {
    std::mem::swap(&mut self.previous, &mut self.current);

    self.current = match self.scanner.next() {
      Some(token_result) => Some(token_result?),
      None => None,
    };

    Ok(())
  }

  fn current(&self) -> &Token {
    self.current.as_ref().unwrap()
  }

  fn previous(&self) -> &Token {
    self.previous.as_ref().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    let scanner = Scanner::new("-(1 + 2) * 2".to_string());

    let mut parser = Parser::new(scanner);

    parser.parse().unwrap();
  }
}
