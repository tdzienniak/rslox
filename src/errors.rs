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

  #[error("unexpected token encountered when parsing expression")]
  UnexpectedTokenInExpression,
}
