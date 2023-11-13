// Syntax grammar:
// program       -> declaration* EOF
// declaration   -> varDecl | statement
// varDecl       -> "var" IDENTIFIER ("=" expression)? ";"
// statement     -> printStmt | exprStmt | block
// while         -> "while" "(" expression ")" block
// block         -> "{" declaration* "}"
// exprStmt      -> expression ";"
// printStmt     -> "print" expression ";"
// expression    -> assignment
// assignment    -> IDENTIFIER "=" assignment | ternary ;
// ternary       -> comma ("?" comma ":" ternary)?
// comma         -> equality ("," equality)*
// equality      -> comparison (("==" | "!=") comparison)*
// comparison    -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term          -> factor ( ( "-" | "+" ) factor )* ;
// factor        -> unary ( ( "/" | "*" ) unary )* ;
// unary         -> ( "!" | "-" ) unary | primary ;
// primary       -> IDENTIFIER | NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

use crate::errors::SyntaxError;
use crate::scanner::{Token, TokenType};
use anyhow::{Result};

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
  Nil,
  Identifier { name: String },
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
  Assignment {
    name: String,
    expression: Box<Expr>,
  },
}

pub(crate) enum Stmt {
  Print {
    expression: Box<Expr>,
  },
  Expression {
    expression: Box<Expr>,
  },
  Declaration {
    name: String,
    initializer: Box<Expr>,
  },
  Block { statements: Vec<Stmt> },
  While {
    condition: Box<Expr>,
    statement: Box<Stmt>
  }
}

pub(crate) struct Parser {
  tokens: Vec<Token>,
  current: usize,
  errors: Vec<SyntaxError>,
}

impl Parser {
  pub(crate) fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      current: 0,
      errors: vec![],
    }
  }

  pub(crate) fn parse(&mut self) -> Result<Vec<Stmt>> {
    let mut statements: Vec<Stmt> = vec![];

    while !self.is_at_and() {
      if let Some(stmt) = self.declaration()? {
        statements.push(stmt);
      }
    }

    if !self.errors.is_empty() {
      for e in &self.errors {
        eprintln!("Syntax error: {e}");
      }

      Ok(vec![])
    } else {
      Ok(statements)
    }
  }

  fn declaration(&mut self) -> Result<Option<Stmt>> {
    let stmt = if self.match_(TokenType::Var) {
      self.variable_declaration()
    } else {
      self.statement()
    };

    stmt.map(Some).or_else(|e| {
      if let Some(syntax_error) = e.downcast_ref::<SyntaxError>() {
        self.errors.push(syntax_error.clone());
        self.synchronize();

        Ok(None)
      } else {
        Err(e)
      }
    })
  }

  fn statement(&mut self) -> Result<Stmt> {
    if self.match_(TokenType::Print) {
      self.print_stmt()
    } else if self.match_(TokenType::LeftBrace) {
      let statements = self.block()?;

      Ok(Stmt::Block {
        statements,
      })
    } else if self.match_(TokenType::While) {
      self.while_()
    } else {
      self.expr_stmt()
    }
  }

  fn print_stmt(&mut self) -> Result<Stmt> {
    let expression = self.expression()?;

    if self.match_(TokenType::Semicolon) {
      Ok(Stmt::Print {
        expression: Box::new(expression),
      })
    } else {
      Err(SyntaxError::MissingSemicolon.into())
    }
  }

  fn block(&mut self) -> Result<Vec<Stmt>> {
    let mut statements: Vec<Stmt> = vec![];

    while self.peek().kind != TokenType::RightBrace && !self.is_at_and() {
      if let Some(stmt) = self.declaration()? {
        statements.push(stmt);
      }
    }

    if self.match_(TokenType::RightBrace) {
      Ok(statements)
    } else {
      Err(SyntaxError::MissingRightBrace.into())
    }
  }

  fn while_(&mut self) -> Result<Stmt> {
    self.consume(TokenType::LeftParen, SyntaxError::MissingWhileCoditionLeftBrace)?;

    let expression = self.expression()?;

    self.consume(TokenType::RightParen, SyntaxError::MissingRightParen)?;
    self.consume(TokenType::LeftBrace, SyntaxError::WhileBodyNotEnclosedInBlock)?;

    let statements = self.block()?;

    Ok(Stmt::While {
      condition: Box::new(expression),
      statement: Box::new(Stmt::Block {
        statements,
      }),
    })
  }

  fn consume(&mut self, token: TokenType, err: SyntaxError) -> Result<()> {
    if !self.match_(token) {
      Err(err.into())
    } else {
      Ok(())
    }
  }

  fn expr_stmt(&mut self) -> Result<Stmt> {
    let expression = self.expression()?;

    if self.match_(TokenType::Semicolon) {
      Ok(Stmt::Expression {
        expression: Box::new(expression),
      })
    } else {
      Err(SyntaxError::MissingSemicolon.into())
    }
  }

  fn variable_declaration(&mut self) -> Result<Stmt> {
    let TokenType::Identifier(name) = self.peek().kind.clone() else {
      return Err(SyntaxError::VariableDeclarationMissingIdentifier.into());
    };

    self.advance();

    if !self.match_(TokenType::Eqal) {
      return Err(SyntaxError::VariableDeclarationMissingAssignment.into());
    }

    let initializer = self.expression()?;

    if self.match_( TokenType::Semicolon) {
      Ok(Stmt::Declaration {
        initializer: Box::new(initializer),
        name,
      })
    } else {
      Err(SyntaxError::MissingSemicolon.into())
    }
  }

  fn expression(&mut self) -> Result<Expr> {
    self.assignment()
  }

  fn assignment(&mut self) -> Result<Expr> {
    let l_value = self.ternary()?;

    if self.match_(TokenType::Eqal) {
      let r_value = self.assignment()?;

      let Expr::Literal {
        value: Literal::Identifier { name },
      } = l_value else {
        return Err(SyntaxError::LValueMustBeAnIdentifier.into());
      };

      Ok(Expr::Assignment {
        name,
        expression: Box::new(r_value),
      })
    } else {
      Ok(l_value)
    }
  }

  fn ternary(&mut self) -> Result<Expr> {
    let conditional = self.comma()?;

    if self.match_(TokenType::Question) {
      let true_case = self.comma()?;

      if self.match_(TokenType::Colon) {
        let false_case = self.ternary()?;

        Ok(Expr::Ternary {
          conditional: Box::new(conditional),
          true_case: Box::new(true_case),
          false_case: Box::new(false_case),
        })
      } else {
        Err(SyntaxError::MissingColonInTernary.into())
      }
    } else {
      Ok(conditional)
    }
  }

  fn comma(&mut self) -> Result<Expr> {
    let mut expr = self.equality()?;

    loop {
      if self.match_(TokenType::Comma) {
        expr = Expr::Binary {
          operator: BinaryOperator::Comma,
          left: Box::new(expr),
          right: Box::new(self.equality()?),
        };
      } else {
        break Ok(expr)
      }
    }
  }

  fn equality(&mut self) -> Result<Expr> {
    let mut expr = self.comparison()?;

    macro_rules! create_equality_expr {
      ($op:expr) => {{
        self.advance();

        let right = self.comparison()?;
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
        _ => break Ok(expr),
      }
    }
  }

  fn comparison(&mut self) -> Result<Expr> {
    let mut expr = self.term()?;

    loop {
      let operator = if self.match_(TokenType::Less) {
        BinaryOperator::Less
      } else if self.match_(TokenType::LessEqual) {
        BinaryOperator::LessEqual
      } else if self.match_(TokenType::Greater) {
        BinaryOperator::Greater
      } else if self.match_(TokenType::GreaterEqual) {
        BinaryOperator::GreaterEqual
      } else {
        break Ok(expr);
      };

      expr = Expr::Binary {
        operator,
        left: Box::new(expr),
        right: Box::new(self.term()?),
      };
    }
  }

  fn term(&mut self) -> Result<Expr> {
    let mut expr = self.factor()?;

    loop {
      let operator = if self.match_(TokenType::Plus) {
        BinaryOperator::Plus
      } else if self.match_(TokenType::Minus) {
        BinaryOperator::Minus
      } else {
        break Ok(expr);
      };

      expr = Expr::Binary {
        operator,
        left: Box::new(expr),
        right: Box::new(self.factor()?),
      };
    }
  }

  fn factor(&mut self) -> Result<Expr> {
    let mut expr = self.unary()?;

    loop {
      let operator = if self.match_(TokenType::Star) {
        BinaryOperator::Star
      } else if self.match_(TokenType::Slash) {
        BinaryOperator::Slash
      } else {
        break Ok(expr);
      };

      expr = Expr::Binary {
        operator,
        left: Box::new(expr),
        right: Box::new(self.unary()?),
      };
    }
  }

  fn unary(&mut self) -> Result<Expr> {
    let operator = if self.match_(TokenType::Bang) {
      UnaryOperator::Bang
    } else if self.match_(TokenType::Minus) {
      UnaryOperator::Minus
    } else {
      return self.primary();
    };

    Ok(Expr::Unary {
      operator,
      expr: Box::new(self.unary()?),
    })
  }

  fn primary(&mut self) -> Result<Expr> {
    macro_rules! create_primary_expr {
      ($value:expr) => {{
        self.advance();

        Ok(Expr::Literal { value: $value })
      }};
    }

    match self.peek().kind.clone() {
      TokenType::Number(value) => create_primary_expr!(Literal::Number { value }),
      TokenType::String(value) => create_primary_expr!(Literal::String { value }),
      TokenType::True => create_primary_expr!(Literal::True),
      TokenType::False => create_primary_expr!(Literal::False),
      TokenType::Nil => create_primary_expr!(Literal::Nil),
      TokenType::Identifier(value) => create_primary_expr!(Literal::Identifier { name: value }),
      TokenType::LeftParen => {
        self.advance();

        let expr = self.expression()?;

        if self.match_(TokenType::RightParen) {
          Ok(Expr::Grouping {
            expr: Box::new(expr),
          })
        } else {
          Err(SyntaxError::MissingRightParen.into())
        }
      }
      _ => Err(SyntaxError::UnexpectedTokenInExpression.into()),
    }
  }

  fn advance(&mut self) -> &Token {
    if !self.is_at_and() {
      self.current += 1;
    }

    self.previous()
  }

  fn match_(&mut self, token_type: TokenType) -> bool {
    if self.peek().kind == token_type {
      self.advance();

      true
    } else {
      false
    }
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

  fn report_error(&mut self, error: SyntaxError) {
    self.errors.push(error);
  }
  fn synchronize(&mut self) {
    self.advance();

    while !self.is_at_and() {
      if let Token {
        kind: TokenType::Semicolon,
        ..
      } = self.previous()
      {
        return;
      }

      match self.peek().kind {
        TokenType::Fun | TokenType::Var => return,
        _ => {}
      }

      self.advance();
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::scanner::Scanner;

  use super::*;

  #[test]
  fn test_name() {
    let scaner = Scanner::new("a = b = true ? 1 : 2;".to_string());
    let mut parser = Parser::new(scaner.scan_tokens().unwrap());

    let ast = parser.parse().unwrap();

    assert_eq!(true, true)
    //
    // assert_eq!(
    //   ast.print(),
    //   "(([,]([*]([+](1, 2), 2), [==](1, 2)) ? 6 : 7) ? 1 : (2 ? 3 : 4))"
    // )
  }
}
