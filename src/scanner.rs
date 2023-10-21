use anyhow::Result;

enum TokenType {
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
    String,
    Number,
  
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

    Eof
  }

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {}


pub(crate) struct Scanner {
    source: String,
}


impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        Scanner {
            source,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        Ok(vec![])
    }
}



