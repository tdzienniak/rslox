use crate::scanner::{Token, TokenType};
use anyhow::Result;

enum BinaryOperator {
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
}

enum UnaryOperator {
  Minus,
  Bang,
}

enum Literal {
  Number { value: f64 },
  String { value: String },
}

enum Expr {
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

macro_rules! match_token {
  ($self:ident, $( $i:pat_param ),*) => {
    match $self.tokens[$self.current] {
      $($i)|* => { Some($self.advance()) },
      _ => None
    }
  };
}

pub(crate) struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  pub(crate) fn new(tokens: Vec<Token>) -> Self {
    Parser { tokens, current: 0 }
  }

  pub(crate) fn parse(self) -> Result<()> {
    Ok(())
  }

  fn expression(&mut self) -> Expr {
    self.equality()
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
      match self.tokens[self.current] {
        Token::Lexeme {
          type_: TokenType::EqualEqual,
          ..
        } => create_equality_expr!(BinaryOperator::EqualEqual),
        Token::Lexeme {
          type_: TokenType::BangEqual,
          ..
        } => create_equality_expr!(BinaryOperator::BangEqual),
        _ => break expr,
      }
    }
  }

  fn comparison(&mut self) -> Expr {
    todo!()
  }

  fn advance(&mut self) -> Token {
    todo!()
  }

  fn peek() -> Token {
    todo!()
  }

  fn previous() -> Token {
    todo!()
  }

  fn isAtEnd() -> bool {
    todo!()
  }
}
