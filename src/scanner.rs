use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
  // Single-character tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or two character tokens
  Bang,
  BangEqual,
  Eqal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Identifier,

  // Keywords
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
  NumberLiteral {
    lexeme: String,
    line: u32,
    value: f64,
  },
  StringLiteral {
    lexeme: String,
    line: u32,
    value: String,
  },
  Lexeme {
    type_: TokenType,
    lexeme: String,
    line: u32,
  },
  EOF,
}

pub(crate) struct Scanner {
  source: String,
  start: usize,
  current: usize,
  line: u32,
}

impl Scanner {
  pub(crate) fn new(source: String) -> Scanner {
    Scanner {
      source,
      start: 0,
      current: 0,
      line: 1,
    }
  }

  fn add_lexeme(&self, type_: TokenType, lexeme: String) -> Token {
    Token::Lexeme {
      type_,
      lexeme,
      line: self.line,
    }
  }

  pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];

    let mut char_iter = self.source.chars().peekable();

    while let Some(char) = char_iter.next() {
      match char {
        '(' => tokens.push(self.add_lexeme(TokenType::LeftParen, char.to_string())),
        ')' => tokens.push(self.add_lexeme(TokenType::RightParen, char.to_string())),
        '{' => tokens.push(self.add_lexeme(TokenType::LeftBrace, char.to_string())),
        '}' => tokens.push(self.add_lexeme(TokenType::RightBrace, char.to_string())),
        ',' => tokens.push(self.add_lexeme(TokenType::Comma, char.to_string())),
        '.' => tokens.push(self.add_lexeme(TokenType::Dot, char.to_string())),
        '-' => tokens.push(self.add_lexeme(TokenType::Minus, char.to_string())),
        '+' => tokens.push(self.add_lexeme(TokenType::Plus, char.to_string())),
        ';' => tokens.push(self.add_lexeme(TokenType::Semicolon, char.to_string())),
        '*' => tokens.push(self.add_lexeme(TokenType::Star, char.to_string())),
        '!' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::BangEqual
          } else {
            TokenType::Bang
          };

          tokens.push(self.add_lexeme(type_, char.to_string()));
        }
        '=' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::EqualEqual
          } else {
            TokenType::Eqal
          };

          tokens.push(self.add_lexeme(type_, char.to_string()));
        }
        '<' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::LessEqual
          } else {
            TokenType::Less
          };

          tokens.push(self.add_lexeme(type_, char.to_string()));
        }
        '>' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::GreaterEqual
          } else {
            TokenType::Greater
          };

          tokens.push(self.add_lexeme(type_, char.to_string()));
        }
        '/' => {
          if char_iter.peek().is_some_and(|c| *c == '/') {
            while char_iter.next_if(|char| *char != '\n').is_some() {}
          } else {
            tokens.push(self.add_lexeme(TokenType::Slash, char.to_string()));
          }
        }
        ' ' | '\r' | '\t' => {}
        '\n' => self.line += 1,
        '"' => {
          let mut value = String::new();

          while let Some(char) = char_iter.next_if(|c| *c != '"') {
            value.push(char);
          }

          // consume the closing "
          char_iter.next();

          tokens.push(Token::StringLiteral {
            lexeme: value.clone(),
            line: self.line,
            value,
          });
        }
        _ => {
          if char.is_ascii_digit() {
            let mut value = String::from(char);

            while let Some(char) = char_iter.next_if(|c| c.is_ascii_digit()) {
              value.push(char);
            }

            if char_iter.peek().is_some_and(|c| *c == '.')
              && char_iter
                .clone()
                .skip(1)
                .peekable()
                .peek()
                .is_some_and(|c| c.is_ascii_control())
            {
              value.push(char_iter.next().unwrap());

              while let Some(char) = char_iter.next_if(|c| c.is_ascii_digit()) {
                value.push(char);
              }
            }

            let literal = value.parse::<f64>()?;

            tokens.push(Token::NumberLiteral {
              lexeme: value,
              line: self.line,
              value: literal,
            })
          } else if char.is_alphabetic() {
            let mut value = String::from(char);

            while let Some(char) = char_iter.next_if(|c| c.is_ascii_alphanumeric()) {
              value.push(char);
            }

            // TODO: add keywords detection

            tokens.push(Token::Lexeme {
              type_: TokenType::Identifier,
              lexeme: value,
              line: self.line,
            })
          }
        }
      }
    }

    tokens.push(Token::EOF);

    Ok(tokens)
  }
}
