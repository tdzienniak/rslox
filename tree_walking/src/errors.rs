use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RuntimeError {
  #[error("expected type {expected:?} given {given:?}")]
  TypeError { expected: String, given: String },

  #[error("undefined: {name:?}")]
  UndefinedIdentifier { name: String },

  #[error("cannot assign to undeclared variable: {identifier:?}")]
  AssignmentToUndeclaredVariable { identifier: String },
}

#[derive(Error, Debug, Clone)]
pub(crate) enum SyntaxError {
  #[error("';' expected at the end of a statement")]
  MissingSemicolon,

  #[error("'var' should be followed by an identifier")]
  VariableDeclarationMissingIdentifier,

  #[error("declared variable must be initialized")]
  VariableDeclarationMissingAssignment,

  #[error("left side of an assignment must be an identifier")]
  LValueMustBeAnIdentifier,

  #[error("missing ':' in conditional expression")]
  MissingColonInTernary,

  #[error("closing paren ')' was not found")]
  MissingRightParen,

  #[error("unexpected token encountered when parsing an expression")]
  UnexpectedTokenInExpression,

  #[error("expected closing '}}' after a block")]
  MissingRightBrace,

  #[error("'while' condition must be enclosed in parens")]
  MissingWhileConditionLeftParen,

  #[error("'while' body must be enclosed in block")]
  WhileBodyNotEnclosedInBlock,

  #[error("'if' condition must be enclosed in parens")]
  MissingIfConditionLeftParen,

  #[error("'if' body must be enclosed in block")]
  IfBodyNotEnclosedInBlock,

  #[error("'else' body must be enclosed in block")]
  ElseBodyNotEnclosedInBlock,

  #[error("parameters must be enclosed in parens")]
  MissingParametersDeclarationOpeningParen,

  #[error("missing function identifier")]
  MissingFunctionDeclarationIdentifier,

  #[error("expected parameter identifier")]
  ExpectedParameterIdentifier,

  #[error("missing function body opening brace")]
  MissingBodyOpeningBrace,
}
