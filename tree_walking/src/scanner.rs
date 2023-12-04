use anyhow::{anyhow, Result};

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
  Colon,
  Question,

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
  Identifier(String),
  Number(f64),
  String(String),

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

  // Other
  Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenType,
  pub lexeme: String,
  pub line: u32,
}

pub(crate) struct Scanner {
  source: String,
  line: u32,
  index: usize,
  was_eof_yielded: bool,
}

impl Scanner {
  pub(crate) fn new(source: String) -> Self {
    Scanner {
      line: 1,
      index: 0,
      source,
      was_eof_yielded: false,
    }
  }

  fn add_token(&mut self, kind: TokenType, lexeme: String) -> Option<Result<Token>> {
    Some(Ok(Token {
      kind,
      lexeme,
      line: self.line,
    }))
  }

  fn slice(&self) -> &str {
    &self.source[self.index..]
  }

  fn peek_char(&self, nth: usize) -> Option<char> {
    let slice = self.slice();

    slice.chars().nth(nth)
  }

  fn next_char(&mut self) -> Option<char> {
    // Slice of leftover characters
    let slice = self.slice();

    // Iterator over leftover characters
    let mut chars = slice.chars();

    // Query the next char
    let next_char = chars.next()?;

    // Compute the new index by looking at how many bytes are left
    // after querying the next char
    self.index = self.source.len() - chars.as_str().len();

    // Return next char
    Some(next_char)
  }

  pub fn next_char_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
    match self.peek_char(0) {
      Some(c) if func(&c) => self.next_char(),
      _ => None
    }
  }

  fn next_token(&mut self) -> Option<Result<Token>> {
    while let Some(char) = self.next_char() {
      match char {
        '(' => return  self.add_token(TokenType::LeftParen, char.to_string()),
        ')' => return self.add_token(TokenType::RightParen, char.to_string()),
        '{' => return self.add_token(TokenType::LeftBrace, char.to_string()),
        '}' => return self.add_token(TokenType::RightBrace, char.to_string()),
        ',' => return self.add_token(TokenType::Comma, char.to_string()),
        '.' => return self.add_token(TokenType::Dot, char.to_string()),
        '-' => return self.add_token(TokenType::Minus, char.to_string()),
        '+' => return self.add_token(TokenType::Plus, char.to_string()),
        ';' => return self.add_token(TokenType::Semicolon, char.to_string()),
        '*' => return self.add_token(TokenType::Star, char.to_string()),
        '?' => return self.add_token(TokenType::Question, char.to_string()),
        ':' => return self.add_token(TokenType::Colon, char.to_string()),
        '!' => {
          let type_ = if self.peek_char(0).is_some_and(|c| c == '=') {
            self.next_char();
            TokenType::BangEqual
          } else {
            TokenType::Bang
          };

          return self.add_token(type_, char.to_string());
        }
        '=' => {
          let type_ = if self.peek_char(0).is_some_and(|c| c == '=') {
           self.next_char();
            TokenType::EqualEqual
          } else {
            TokenType::Eqal
          };

          return self.add_token(type_, char.to_string());
        }
        '<' => {
          let type_ = if self.peek_char(0).is_some_and(|c| c == '=') {
           self.next_char();
            TokenType::LessEqual
          } else {
            TokenType::Less
          };

          return self.add_token(type_, char.to_string());
        }
        '>' => {
          let type_ = if self.peek_char(0).is_some_and(|c| c == '=') {
           self.next_char();
            TokenType::GreaterEqual
          } else {
            TokenType::Greater
          };

          return self.add_token(type_, char.to_string());
        }
        '/' => {
          if self.peek_char(0).is_some_and(|c| c == '/') {
            while self.next_char_if(|char| *char != '\n').is_some() {}
          } else {
            return self.add_token(TokenType::Slash, char.to_string());
          }
        }
        ' ' | '\r' | '\t' => {}
        '\n' => self.line += 1,
        '"' => {
          let mut value = String::new();

          while let Some(char) = self.next_char_if(|c| *c != '"') {
            value.push(char);
          }

          // consume the closing "
          self.next_char();

          return self.add_token(TokenType::String(value.clone()), value);
        }
        _ => {
          if char.is_ascii_digit() {
            let mut value = String::from(char);

            while let Some(char) = self.next_char_if(|c| c.is_ascii_digit()) {
              value.push(char);
            }

            if self.peek_char(0).is_some_and(|c| c == '.')
              && self.peek_char(1)
                .is_some_and(|c| c.is_ascii_digit())
            {
              value.push(self.next_char().unwrap());

              while let Some(char) = self.next_char_if(|c| c.is_ascii_digit()) {
                value.push(char);
              }
            }

            return if let Ok(parsed) = value.parse::<f64>() {
              self.add_token(TokenType::Number(parsed), value.clone())
            } else {
              Some(Err(anyhow!("cannot parse string into number")))
            }

          } else if char.is_alphabetic() {
            let mut value = String::from(char);

            while let Some(char) = self.next_char_if(|c| c.is_ascii_alphanumeric()) {
              value.push(char);
            }

            let token_type = match value.as_str() {
              "if" => TokenType::If,
              "else" => TokenType::Else,
              "true" => TokenType::True,
              "false" => TokenType::False,
              "nil" => TokenType::Nil,
              "while" => TokenType::While,
              "for" => TokenType::For,
              "and" => TokenType::And,
              "or" => TokenType::Or,
              "fun" => TokenType::Fun,
              "return" => TokenType::Return,
              "class" => TokenType::Class,
              "this" => TokenType::This,
              "super" => TokenType::Super,
              "var" => TokenType::Var,
              "print" => TokenType::Print,
              _ => TokenType::Identifier(value.clone()),
            };

            return self.add_token(token_type, value);
          }
        }
      }
    }

    if self.was_eof_yielded {
      None
    } else {
      self.was_eof_yielded = true;
      self.add_token(TokenType::Eof, "".to_string())
    }
  }
}

impl Iterator for Scanner {
  type Item = Result<Token>;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_token()
  }
}
