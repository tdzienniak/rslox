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
  Eof
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
  pub(crate) kind: TokenType,
  pub(crate)line: u32
}

pub(crate) struct Scanner {
  source: String,
  start: usize,
  current: usize,
  line: u32,
  tokens: Vec<Token>
}

impl Scanner {
  pub(crate) fn new(source: String) -> Scanner {
    Scanner {
      source,
      start: 0,
      current: 0,
      line: 1,
      tokens: vec![]
    }
  }

  fn add_token(&mut self, kind: TokenType) {
    self.tokens.push(Token {
      kind,
      line: self.line,
    });
  }

  pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];

    let cloned_source = self.source.clone();
    let mut char_iter = cloned_source.chars().peekable();

    while let Some(char) = char_iter.next() {
      match char {
        '(' => self.add_token(TokenType::LeftParen),
        ')' => self.add_token(TokenType::RightParen),
        '{' => self.add_token(TokenType::LeftBrace),
        '}' => self.add_token(TokenType::RightBrace),
        ',' => self.add_token(TokenType::Comma),
        '.' => self.add_token(TokenType::Dot),
        '-' => self.add_token(TokenType::Minus),
        '+' => self.add_token(TokenType::Plus),
        ';' => self.add_token(TokenType::Semicolon),
        '*' => self.add_token(TokenType::Star),
        '!' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::BangEqual
          } else {
            TokenType::Bang
          };

          self.add_token(type_);
        }
        '=' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::EqualEqual
          } else {
            TokenType::Eqal
          };

          self.add_token(type_);
        }
        '<' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::LessEqual
          } else {
            TokenType::Less
          };

          self.add_token(type_);
        }
        '>' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::GreaterEqual
          } else {
            TokenType::Greater
          };

          self.add_token(type_);
        }
        '/' => {
          if char_iter.peek().is_some_and(|c| *c == '/') {
            while char_iter.next_if(|char| *char != '\n').is_some() {}
          } else {
            self.add_token(TokenType::Slash);
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

          self.add_token(TokenType::String(value));
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

            self.add_token(TokenType::Number(value.parse::<f64>()?));
          } else if char.is_alphabetic() {
            let mut value = String::from(char);

            while let Some(char) = char_iter.next_if(|c| c.is_ascii_alphanumeric()) {
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
              _ => TokenType::Identifier(value),
            };
            
            self.add_token(token_type);
          }
        }
      }
    }

    self.add_token(TokenType::Eof);

    Ok(self.tokens)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    let tokens = Scanner::new("print \"Hello World!\"".to_string());

    assert_eq!(
      tokens.scan_tokens().unwrap(),
      vec![
        Token {
          kind: TokenType::Print,
          line: 1,
        },
        Token {
          kind: TokenType::String("Hello World!".to_string()),
          line: 1,
        },
        Token {
          kind: TokenType::Eof,
          line: 1
        },
      ]
    );
  }
}
