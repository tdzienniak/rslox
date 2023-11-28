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
pub(crate) struct Token {
  pub(crate) kind: TokenType,
  pub(crate) lexeme: String,
  pub(crate) line: u32,
}

pub(crate) struct Scanner {
  source: String,
  line: u32,
  tokens: Vec<Token>,
}

impl Scanner {
  pub(crate) fn new(source: String) -> Scanner {
    Scanner {
      source,
      line: 1,
      tokens: vec![],
    }
  }

  fn add_token(&mut self, kind: TokenType, lexeme: String) {
    self.tokens.push(Token {
      kind,
      lexeme,
      line: self.line,
    });
  }

  pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>> {
    let cloned_source = self.source.clone();
    let mut char_iter = cloned_source.chars().peekable();

    while let Some(char) = char_iter.next() {
      match char {
        '(' => self.add_token(TokenType::LeftParen, char.to_string()),
        ')' => self.add_token(TokenType::RightParen, char.to_string()),
        '{' => self.add_token(TokenType::LeftBrace, char.to_string()),
        '}' => self.add_token(TokenType::RightBrace, char.to_string()),
        ',' => self.add_token(TokenType::Comma, char.to_string()),
        '.' => self.add_token(TokenType::Dot, char.to_string()),
        '-' => self.add_token(TokenType::Minus, char.to_string()),
        '+' => self.add_token(TokenType::Plus, char.to_string()),
        ';' => self.add_token(TokenType::Semicolon, char.to_string()),
        '*' => self.add_token(TokenType::Star, char.to_string()),
        '?' => self.add_token(TokenType::Question, char.to_string()),
        ':' => self.add_token(TokenType::Colon, char.to_string()),
        '!' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::BangEqual
          } else {
            TokenType::Bang
          };

          self.add_token(type_, char.to_string());
        }
        '=' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::EqualEqual
          } else {
            TokenType::Eqal
          };

          self.add_token(type_, char.to_string());
        }
        '<' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::LessEqual
          } else {
            TokenType::Less
          };

          self.add_token(type_, char.to_string());
        }
        '>' => {
          let type_ = if char_iter.peek().is_some_and(|c| *c == '=') {
            char_iter.next();
            TokenType::GreaterEqual
          } else {
            TokenType::Greater
          };

          self.add_token(type_, char.to_string());
        }
        '/' => {
          if char_iter.peek().is_some_and(|c| *c == '/') {
            while char_iter.next_if(|char| *char != '\n').is_some() {}
          } else {
            self.add_token(TokenType::Slash, char.to_string());
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

          self.add_token(TokenType::String(value.clone()), value);
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

            self.add_token(TokenType::Number(value.parse::<f64>()?), value.clone());
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
              _ => TokenType::Identifier(value.clone()),
            };

            self.add_token(token_type, value);
          }
        }
      }
    }

    self.add_token(TokenType::Eof, "".to_string());

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
          lexeme: "print".to_string(),
          line: 1,
        },
        Token {
          kind: TokenType::String("Hello World!".to_string()),
          lexeme: "Hello World!".to_string(),
          line: 1,
        },
        Token {
          kind: TokenType::Eof,
          lexeme: "".to_string(),
          line: 1
        },
      ]
    );
  }
}
