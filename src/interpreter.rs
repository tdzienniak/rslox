use crate::parser::{BinaryOperator, Expr, Literal, UnaryOperator};
use anyhow::{anyhow, Result};

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RuntimeError {
  #[error("expected type")]
  TypeError { expected: String, given: String },
}

#[derive(Debug)]
pub(crate) struct NumberValue(f64);

#[derive(Debug)]
pub(crate) struct StringValue(String);

#[derive(Debug)]
pub(crate) struct BoolValue(bool);

#[derive(Debug)]
pub(crate) enum Value {
  Number(NumberValue),
  String(StringValue),
  Bool(BoolValue),
  Nil,
}

impl Value {
  fn type_as_string(&self) -> String {
    match self {
      Value::Bool(_) => "bool".to_string(),
      Value::Number(_) => "number".to_string(),
      Value::String(_) => "string".to_string(),
      Value::Nil => "nil".to_string(),
    }
  }

  fn is_truthy(&self) -> bool {
    match self {
      Value::Bool(inner) => inner.0,
      _ => true,
    }
  }

  fn is_equal(&self, other: &Value) -> Result<bool> {
    match (self, other) {
      (Value::Bool(v1), Value::Bool(v2)) => Ok(v1.0 == v2.0),
      (Value::Number(v1), Value::Number(v2)) => Ok(v1.0 == v2.0),
      (Value::String(v1), Value::String(v2)) => Ok(v1.0 == v2.0),
      _ => Err(anyhow!("todo")),
    }
  }

  fn is_greater_than(&self, other: &Value) -> Result<bool> {
    match (self, other) {
      (Value::Number(v1), Value::Number(v2)) => Ok(v1.0 > v2.0),
      _ => Err(anyhow!("todo")),
    }
  }

  fn is_lesser_than(&self, other: &Value) -> Result<bool> {
    match (self, other) {
      (Value::Number(v1), Value::Number(v2)) => Ok(v1.0 < v2.0),
      _ => Err(anyhow!("todo")),
    }
  }
}

pub(crate) trait Interpret {
  fn interpret(&self) -> Result<Value>;
}

impl Interpret for Expr {
  fn interpret(&self) -> Result<Value> {
    match self {
      Expr::Unary { operator, expr } => {
        let value = expr.interpret()?;
        match operator {
          UnaryOperator::Bang => {
            if let Value::Bool(inner) = value {
              Ok(Value::Bool(BoolValue(!inner.0)))
            } else {
              Err(
                RuntimeError::TypeError {
                  expected: "bool".to_string(),
                  given: value.type_as_string(),
                }
                .into(),
              )
            }
          }
          UnaryOperator::Minus => {
            if let Value::Number(inner) = value {
              Ok(Value::Number(NumberValue(-inner.0)))
            } else {
              Err(
                RuntimeError::TypeError {
                  expected: "number".to_string(),
                  given: value.type_as_string(),
                }
                .into(),
              )
            }
          }
        }
      }
      Expr::Binary {
        operator,
        left,
        right,
      } => {
        let left_value = left.interpret()?;
        let right_value = right.interpret()?;

        match operator {
          BinaryOperator::BangEqual => {
            Ok(Value::Bool(BoolValue(!left_value.is_equal(&right_value)?)))
          }
          BinaryOperator::Comma => Ok(right_value),
          BinaryOperator::EqualEqual => {
            Ok(Value::Bool(BoolValue(left_value.is_equal(&right_value)?)))
          }
          BinaryOperator::Plus => match (&left_value, &right_value) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(NumberValue(v1.0 + v2.0))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Minus => match (&left_value, &right_value) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(NumberValue(v1.0 - v2.0))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Star => match (&left_value, &right_value) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(NumberValue(v1.0 * v2.0))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Slash => match (&left_value, &right_value) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Value::Number(NumberValue(v1.0 + v2.0))),
            _ => Err(anyhow!("todo")),
          },
          _ => Err(anyhow!("todo")),
        }
      }
      Expr::Ternary {
        conditional,
        true_case,
        false_case,
      } => {
        let conditional_value = conditional.interpret()?;

        if conditional_value.is_truthy() {
          true_case.interpret()
        } else {
          false_case.interpret()
        }
      }
      Expr::Grouping { expr } => expr.interpret(),
      Expr::Literal { value } => Ok(match value {
        Literal::True => Value::Bool(BoolValue(true)),
        Literal::False => Value::Bool(BoolValue(false)),
        Literal::Number { value } => Value::Number(NumberValue(*value)),
        Literal::String { value } => Value::String(StringValue(value.clone())),
        Literal::Nil => Value::Nil,
        Literal::Identifier { .. } => todo!("implement environments")
      }),
      Expr::Assignment { name, expression} => {
        todo!("implement assignment")
      }
    }
  }
}
