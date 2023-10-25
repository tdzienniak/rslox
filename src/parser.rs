/*t
Syntax grammar:
expression    -> ternary
ternary       -> comma ("?" comma ":" ternary)?
comma         -> equality ("," equality)*
equality      -> comparison (("==" | "!=") comparison)*
comparison    -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term          -> factor ( ( "-" | "+" ) factor )* ;
factor        -> unary ( ( "/" | "*" ) unary )* ;
unary         -> ( "!" | "-" ) unary | primary ;
primary       -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
*/

use crate::scanner::{Token, TokenType};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BinaryOperator {
  EqualEqual,
  BangEqual,
  Plus,
  Minus,
  Slash,
  Star,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UnaryOperator {
  Minus,
  Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
  Number { value: f64 },
  String { value: String },
  True,
  False,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
  Ternary {
    conditional: Box<Expr>,
    true_case: Box<Expr>,
    false_case: Box<Expr>,
  },
  Binary {
    operator: BinaryOperator,
    left: Box<Expr>,
    right: Box<Expr>,
  },
  Unary {
    operator: UnaryOperator,
    expr: Box<Expr>,
  },
  Grouping {
    expr: Box<Expr>,
  },
  Literal {
    value: Literal,
  },
}

pub(crate) struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  pub(crate) fn new(tokens: Vec<Token>) -> Self {
    Parser { tokens, current: 0 }
  }

  pub(crate) fn parse(&mut self) -> Result<Expr> {
    Ok(self.expression())
  }

  fn expression(&mut self) -> Expr {
    self.ternary()
  }

  fn ternary(&mut self) -> Expr {
    let conditional = self.comma();

    if self.peek().kind == TokenType::Question {
      self.advance();
      let true_case = self.comma();

      if self.peek().kind == TokenType::Colon {
        self.advance();
        let false_case = self.ternary();

        Expr::Ternary {
          conditional: Box::new(conditional),
          true_case: Box::new(true_case),
          false_case: Box::new(false_case),
        }
      } else {
        panic!("colon expected");
      }
    } else {
      conditional
    }
  }

  fn comma(&mut self) -> Expr {
    let mut expr = self.equality();

    macro_rules! create_comma_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.equality();
        expr = Expr::Binary {
          operator: $op,
          left: Box::new(expr),
          right: Box::new(right),
        }
      }};
    }

    loop {
      match self.peek().kind {
        TokenType::Comma => create_comma_expr!(BinaryOperator::Comma),
        _ => break expr,
      }
    }
  }

  fn equality(&mut self) -> Expr {
    let mut expr = self.comparison();

    macro_rules! create_equality_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.comparison();
        expr = Expr::Binary {
          operator: $op,
          left: Box::new(expr),
          right: Box::new(right),
        }
      }};
    }

    loop {
      match self.peek().kind {
        TokenType::EqualEqual => create_equality_expr!(BinaryOperator::EqualEqual),
        TokenType::BangEqual => create_equality_expr!(BinaryOperator::BangEqual),
        _ => break expr,
      }
    }
  }

  fn comparison(&mut self) -> Expr {
    let mut expr = self.term();

    macro_rules! create_comparison_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.term();
        expr = Expr::Binary {
          operator: $op,
          left: Box::new(expr),
          right: Box::new(right),
        }
      }};
    }

    loop {
      match self.peek().kind {
        TokenType::Less => create_comparison_expr!(BinaryOperator::Less),
        TokenType::LessEqual => create_comparison_expr!(BinaryOperator::LessEqual),
        TokenType::Greater => create_comparison_expr!(BinaryOperator::Greater),
        TokenType::GreaterEqual => create_comparison_expr!(BinaryOperator::GreaterEqual),
        _ => break expr,
      }
    }
  }

  fn term(&mut self) -> Expr {
    let mut expr = self.factor();

    macro_rules! create_term_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.factor();
        expr = Expr::Binary {
          operator: $op,
          left: Box::new(expr),
          right: Box::new(right),
        }
      }};
    }

    loop {
      match self.peek().kind {
        TokenType::Plus => create_term_expr!(BinaryOperator::Plus),
        TokenType::Minus => create_term_expr!(BinaryOperator::Minus),
        _ => break expr,
      }
    }
  }

  fn factor(&mut self) -> Expr {
    let mut expr = self.unary();

    macro_rules! create_factor_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.unary();
        expr = Expr::Binary {
          operator: $op,
          left: Box::new(expr),
          right: Box::new(right),
        }
      }};
    }

    loop {
      match self.peek().kind {
        TokenType::Star => create_factor_expr!(BinaryOperator::Star),
        TokenType::Slash => create_factor_expr!(BinaryOperator::Slash),
        _ => break expr,
      }
    }
  }

  fn unary(&mut self) -> Expr {
    macro_rules! create_unary_expr {
      ($op:expr) => {{
        self.advance();

        let expr = self.unary();
        Expr::Unary {
          operator: $op,
          expr: Box::new(expr),
        }
      }};
    }

    match self.peek().kind {
      TokenType::Bang => create_unary_expr!(UnaryOperator::Bang),
      TokenType::Minus => create_unary_expr!(UnaryOperator::Minus),
      _ => self.primary(),
    }
  }

  fn primary(&mut self) -> Expr {
    macro_rules! create_primary_expr {
      ($value:expr) => {{
        self.advance();

        Expr::Literal { value: $value }
      }};
    }

    match self.peek().kind.clone() {
      TokenType::Number(value) => create_primary_expr!(Literal::Number { value }),
      TokenType::String(value) => create_primary_expr!(Literal::String { value }),
      TokenType::True => create_primary_expr!(Literal::True),
      TokenType::False => create_primary_expr!(Literal::False),
      TokenType::LeftParen => {
        self.advance();

        let expr = self.expression();

        if self.peek().kind == TokenType::RightParen {
          self.advance();

          Expr::Grouping {
            expr: Box::new(expr),
          }
        } else {
          panic!("wrong")
        }
      }
      _ => {
        panic!("wrong")
      }
    }
  }

  fn advance(&mut self) -> &Token {
    if !self.is_at_and() {
      self.current += 1;
    }

    self.previous()
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.current]
  }

  fn previous(&mut self) -> &Token {
    &self.tokens[self.current - 1]
  }

  fn is_at_and(&self) -> bool {
    self.peek().kind == TokenType::Eof
  }
}

#[cfg(test)]
mod tests {
  use crate::scanner::Scanner;

  use super::*;

  #[test]
  fn test_name() {
    let scaner = Scanner::new("((1 + 2) * 2, 1 == 2 ? 6 : 7) ? 1 : 2 ? 3 : 4".to_string());
    let mut parser = Parser::new(scaner.scan_tokens().unwrap());

    let ast = parser.parse().unwrap();

    println!("{:?}", ast)
  }
}
