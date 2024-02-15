// Syntax grammar:
// program       -> declaration* EOF
// declaration   -> varDecl | statement
// funDecl       -> "fun" function
// function      -> IDENTIFIER "(" parameters? ")" block
// parameters    -> IDENTIFIER ("," IDENTIFIER)*
// varDecl       -> "var" IDENTIFIER ("=" expression)? ";"
// statement     -> exprStmt | block | while | if
// while         -> "while" "(" expression ")" block
// if            -> "if" "(" expression ")" block ("else" block)?
// block         -> "{" declaration* "}"
// exprStmt      -> expression ";"
// expression    -> comma;
// comma         -> assignment ("," assignment)*
// assignment    -> IDENTIFIER "=" assignment | logical_or;
// logical_or    -> logical_and ("or" logical_and)*
// logical_and   -> ternary ("and" ternary)*
// ternary       -> equality ("?" equality ":" ternary)?
// equality      -> comparison (("==" | "!=") comparison)*
// comparison    -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term          -> factor ( ( "-" | "+" ) factor )* ;
// factor        -> unary ( ( "/" | "*" ) unary )* ;
// unary         -> ( "!" | "-" ) unary | call ;
// call          -> primary ("(" arguments ")")*
// arguments     -> expression ("," expression)*
// primary       -> IDENTIFIER | NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

use crate::errors::SyntaxError;
use anyhow::Result;
use scanner::{Token, TokenType};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
  COUNTER.fetch_add(1, Ordering::Relaxed)
}

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
  Or,
  And,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UnaryOperator {
  Minus,
  Bang,
}

#[derive(Debug, Clone)]
pub(crate) enum Literal {
  Number { value: f64 },
  String { value: String },
  True,
  False,
  Nil,
  Identifier { name: String, id: usize },
}

#[derive(Debug, Clone)]
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
    id: usize,
  },
  Call {
    function: Box<Expr>,
    arguments: Vec<Expr>,
  },
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
  Expression {
    expression: Box<Expr>,
  },
  Declaration {
    name: String,
    initializer: Box<Expr>,
  },
  FunDeclaration {
    name: String,
    parameters: Vec<String>,
    body: Vec<Stmt>,
  },
  Block {
    statements: Vec<Stmt>,
  },
  While {
    condition: Box<Expr>,
    statement: Box<Stmt>,
  },
  If {
    condition: Box<Expr>,
    true_case: Box<Stmt>,
    false_case: Option<Box<Stmt>>,
  },
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
    } else if self.match_(TokenType::Fun) {
      self.function_declaration()
    } else {
      self.statement()
    };

    stmt.map(Some).or_else(|e| {
      if let Some(syntax_error) = e.downcast_ref::<SyntaxError>() {
        self.report_error(syntax_error.to_owned());
        self.synchronize();

        Ok(None)
      } else {
        Err(e)
      }
    })
  }

  fn function_declaration(&mut self) -> Result<Stmt> {
    let name = {
      let TokenType::Identifier(ref identifier) = self.peek().kind else {
        return Err(SyntaxError::MissingFunctionDeclarationIdentifier.into());
      };

      identifier.clone()
    };

    self.advance();

    self.consume(
      TokenType::LeftParen,
      SyntaxError::MissingParametersDeclarationOpeningParen,
    )?;

    let parameters = if !self.match_(TokenType::RightParen) {
      let parameters = self.parameters()?;

      self.consume(
        TokenType::RightParen,
        SyntaxError::MissingParametersDeclarationOpeningParen,
      )?;

      parameters
    } else {
      vec![]
    };

    self.consume(TokenType::LeftBrace, SyntaxError::MissingBodyOpeningBrace)?;

    let body = self.block()?;

    Ok(Stmt::FunDeclaration {
      name: name.clone(),
      body,
      parameters,
    })
  }

  fn parameters(&mut self) -> Result<Vec<String>> {
    let mut parameters: Vec<String> = vec![self.match_parameter_identifier()?];

    loop {
      if self.match_(TokenType::Comma) {
        parameters.push(self.match_parameter_identifier()?)
      } else {
        break Ok(parameters);
      }
    }
  }

  fn match_parameter_identifier(&mut self) -> Result<String> {
    let identifier = {
      let TokenType::Identifier(ref identifier) = self.peek().kind else {
        return Err(SyntaxError::ExpectedParameterIdentifier.into());
      };

      identifier.clone()
    };

    self.advance();

    Ok(identifier)
  }

  fn statement(&mut self) -> Result<Stmt> {
    if self.match_(TokenType::LeftBrace) {
      let statements = self.block()?;

      Ok(Stmt::Block { statements })
    } else if self.match_(TokenType::While) {
      self.while_()
    } else if self.match_(TokenType::If) {
      self.if_()
    } else {
      self.expr_stmt()
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
    self.consume(
      TokenType::LeftParen,
      SyntaxError::MissingWhileConditionLeftParen,
    )?;

    let expression = self.expression()?;

    self.consume(TokenType::RightParen, SyntaxError::MissingRightParen)?;
    self.consume(
      TokenType::LeftBrace,
      SyntaxError::WhileBodyNotEnclosedInBlock,
    )?;

    let statements = self.block()?;

    Ok(Stmt::While {
      condition: Box::new(expression),
      statement: Box::new(Stmt::Block { statements }),
    })
  }

  fn if_(&mut self) -> Result<Stmt> {
    self.consume(
      TokenType::LeftParen,
      SyntaxError::MissingIfConditionLeftParen,
    )?;

    let condition = self.expression()?;

    self.consume(TokenType::RightParen, SyntaxError::MissingRightParen)?;
    self.consume(TokenType::LeftBrace, SyntaxError::IfBodyNotEnclosedInBlock)?;

    let true_case = self.block()?;

    let else_case = if self.match_(TokenType::Else) {
      self.consume(
        TokenType::LeftBrace,
        SyntaxError::ElseBodyNotEnclosedInBlock,
      )?;

      let statements = self.block()?;

      Some(Stmt::Block { statements })
    } else {
      None
    };

    Ok(Stmt::If {
      condition: Box::new(condition),
      true_case: Box::new(Stmt::Block {
        statements: true_case,
      }),
      false_case: else_case.map(Box::new),
    })
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

    if self.match_(TokenType::Semicolon) {
      Ok(Stmt::Declaration {
        initializer: Box::new(initializer),
        name,
      })
    } else {
      Err(SyntaxError::MissingSemicolon.into())
    }
  }

  fn expression(&mut self) -> Result<Expr> {
    self.comma()
  }

  fn assignment(&mut self) -> Result<Expr> {
    let l_value = self.logical_or()?;

    if self.match_(TokenType::Eqal) {
      let r_value = self.assignment()?;

      let Expr::Literal {
        value: Literal::Identifier { name, .. },
      } = l_value
      else {
        return Err(SyntaxError::LValueMustBeAnIdentifier.into());
      };

      Ok(Expr::Assignment {
        name,
        expression: Box::new(r_value),
        id: get_id(),
      })
    } else {
      Ok(l_value)
    }
  }

  fn logical_or(&mut self) -> Result<Expr> {
    let mut expr = self.logical_and()?;

    loop {
      if self.match_(TokenType::Or) {
        expr = Expr::Binary {
          operator: BinaryOperator::Or,
          left: Box::new(expr),
          right: Box::new(self.logical_and()?),
        };
      } else {
        break Ok(expr);
      };
    }
  }

  fn logical_and(&mut self) -> Result<Expr> {
    let mut expr = self.ternary()?;

    loop {
      if self.match_(TokenType::And) {
        expr = Expr::Binary {
          operator: BinaryOperator::And,
          left: Box::new(expr),
          right: Box::new(self.ternary()?),
        };
      } else {
        break Ok(expr);
      };
    }
  }

  fn ternary(&mut self) -> Result<Expr> {
    let conditional = self.equality()?;

    if self.match_(TokenType::Question) {
      let true_case = self.equality()?;

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
    let mut expr = self.assignment()?;

    loop {
      if self.match_(TokenType::Comma) {
        expr = Expr::Binary {
          operator: BinaryOperator::Comma,
          left: Box::new(expr),
          right: Box::new(self.assignment()?),
        };
      } else {
        break Ok(expr);
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

        Expr::Literal { value: $value }
      }};
    }

    let mut primary = match self.peek().kind.clone() {
      TokenType::Number(value) => create_primary_expr!(Literal::Number { value }),
      TokenType::String(value) => create_primary_expr!(Literal::String { value }),
      TokenType::True => create_primary_expr!(Literal::True),
      TokenType::False => create_primary_expr!(Literal::False),
      TokenType::Nil => create_primary_expr!(Literal::Nil),
      TokenType::Identifier(value) => create_primary_expr!(Literal::Identifier {
        name: value,
        id: get_id()
      }),
      TokenType::LeftParen => {
        self.advance();

        let expr = self.expression()?;

        if self.match_(TokenType::RightParen) {
          Expr::Grouping {
            expr: Box::new(expr),
          }
        } else {
          return Err(SyntaxError::MissingRightParen.into());
        }
      }
      _ => return Err(SyntaxError::UnexpectedTokenInExpression.into()),
    };

    loop {
      if self.match_(TokenType::LeftParen) {
        let arguments = self.finish_call()?;

        primary = Expr::Call {
          function: Box::new(primary),
          arguments,
        }
      } else {
        break Ok(primary);
      }
    }
  }

  fn finish_call(&mut self) -> Result<Vec<Expr>> {
    let mut arguments: Vec<Expr> = vec![];

    if self.match_(TokenType::RightParen) {
      return Ok(arguments);
    }

    loop {
      arguments.push(self.assignment()?);

      if !self.match_(TokenType::Comma) {
        break;
      }
    }

    self.consume(TokenType::RightParen, SyntaxError::MissingRightParen)?;

    Ok(arguments)
  }

  fn consume(&mut self, token: TokenType, err: SyntaxError) -> Result<()> {
    if !self.match_(token) {
      Err(err.into())
    } else {
      Ok(())
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
  use scanner::Scanner;
  use crate::ast_printer::Printer;

  use super::*;

  #[test]
  fn test_name() {
    let scaner = Scanner::new("test()(1, 2);".to_string());
    let mut parser = Parser::new(scaner.collect::<Result<Vec<Token>>>().unwrap());

    let ast = parser.parse().unwrap();

    assert_eq!(ast[0].print(), "")
    //
    // assert_eq!(
    //   ast.print(),
    //   "(([,]([*]([+](1, 2), 2), [==](1, 2)) ? 6 : 7) ? 1 : (2 ? 3 : 4))"
    // )
  }
}
