use std::rc::Rc;
use crate::environment::Environment;
use crate::parser::{BinaryOperator, Expr, Literal, Stmt, UnaryOperator};
use anyhow::{anyhow, Result};

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RuntimeError {
  #[error("expected type {expected:?} given {given:?}")]
  TypeError { expected: String, given: String },

  #[error("undefined: {name:?}")]
  UndefinedIdentifier { name: String },
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

pub(crate) trait Interpret<T> {
  fn interpret(&self, environment: &mut Environment) -> Result<T>;
}

impl Interpret<Rc<Value>> for Expr {
  fn interpret(&self, environment: &mut Environment) -> Result<Rc<Value>> {
    match self {
      Expr::Unary { operator, expr } => {
        let value = expr.interpret(environment)?;
        match operator {
          UnaryOperator::Bang => {
            if let Value::Bool(inner) = value.as_ref() {
              Ok(Rc::new(Value::Bool(BoolValue(!inner.0))))
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
            if let Value::Number(inner) = value .as_ref(){
              Ok(Rc::new(Value::Number(NumberValue(-inner.0))))
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
        let left_value = left.interpret(environment)?;
        let right_value = right.interpret(environment)?;

        match operator {
          BinaryOperator::BangEqual => {
            Ok(Rc::new(Value::Bool(BoolValue(!left_value.is_equal(&right_value)?))))
          }
          BinaryOperator::Comma => Ok(right_value),
          BinaryOperator::EqualEqual => {
            Ok(Rc::new(Value::Bool(BoolValue(left_value.is_equal(&right_value)?))))
          }
          BinaryOperator::Plus => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Rc::new(Value::Number(NumberValue(v1.0 + v2.0)))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Minus => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Rc::new(Value::Number(NumberValue(v1.0 - v2.0)))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Star => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Rc::new(Value::Number(NumberValue(v1.0 * v2.0)))),
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Slash => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => Ok(Rc::new(Value::Number(NumberValue(v1.0 + v2.0)))),
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
        let conditional_value = conditional.interpret(environment)?;

        if conditional_value.is_truthy() {
          true_case.interpret(environment)
        } else {
          false_case.interpret(environment)
        }
      }
      Expr::Grouping { expr } => expr.interpret(environment),
      Expr::Literal { value } => match value {
        Literal::True => Ok(Value::Bool(BoolValue(true)).into()),
        Literal::False => Ok(Value::Bool(BoolValue(false)).into()),
        Literal::Number { value } => Ok(Value::Number(NumberValue(*value)).into()),
        Literal::String { value } => Ok(Value::String(StringValue(value.clone())).into()),
        Literal::Nil => Ok(Value::Nil.into()),
        Literal::Identifier { name } => environment.get(name).ok_or(
          RuntimeError::UndefinedIdentifier {
            name: name.to_string(),
          }
          .into(),
        ),
      },
      Expr::Assignment { name, expression } => {
        todo!("implement assignment")
      }
    }
  }
}

impl Interpret<()> for Stmt {
  fn interpret(&self, environment: &mut Environment) -> Result<()> {
    match self {
      Stmt::Print { expression } => {
        let value = expression.interpret(environment)?;

        let value_str = match value.as_ref() {
          Value::Number(value) => value.0.to_string(),
          Value::String(value) => value.0.clone(),
          Value::Bool(value) => value.0.to_string(),
          Value::Nil => "nil".to_string()
        };

        println!("{}", value_str);

        Ok(())
      }
      Stmt::Expression { expression } => {
        expression.interpret(environment)?;

        Ok(())
      }
      Stmt::Declaration { name, initializer } => {
        let value = initializer.interpret(environment)?;

        environment.define(name, value);

        Ok(())
      }
    }
  }
}
