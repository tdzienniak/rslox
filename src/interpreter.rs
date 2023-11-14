use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::parser::{BinaryOperator, Expr, Literal, Stmt, UnaryOperator};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(crate) struct NumberValue(f64);

#[derive(Debug)]
pub(crate) struct StringValue(String);

#[derive(Debug)]
pub(crate) struct BoolValue(bool);

pub(crate) trait Callable {
  fn call(&self, arguments: Vec<Rc<Value>>) -> Result<Rc<Value>>;
}

pub(crate) struct NativeClock;

impl Callable for NativeClock {
  fn call(&self, _arguments: Vec<Rc<Value>>) -> Result<Rc<Value>> {
    let start = SystemTime::now();
    let since_the_epoch = start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
    Ok(Rc::new(Value::Number(NumberValue(since_the_epoch.as_secs_f64()))))
  }
}

pub(crate) struct NativePrint;

impl Callable for NativePrint {
  fn call(&self, _arguments: Vec<Rc<Value>>) -> Result<Rc<Value>> {
    println!("{}", _arguments.iter().map(|value| match value.as_ref() {
      Value::Number(value) => value.0.to_string(),
      Value::String(value) => value.0.clone(),
      Value::Bool(value) => value.0.to_string(),
      Value::Nil => "nil".to_string(),
      Value::Function(_) => "function".to_string(),
    }).collect::<Vec<String>>().join(" "));

    Ok(Rc::new(Value::Nil))
  }
}

pub(crate) enum Value {
  Number(NumberValue),
  String(StringValue),
  Bool(BoolValue),
  Nil,
  Function(Box<dyn Callable>),
}

impl Value {
  fn type_as_string(&self) -> String {
    match self {
      Value::Bool(_) => "bool".to_string(),
      Value::Number(_) => "number".to_string(),
      Value::String(_) => "string".to_string(),
      Value::Nil => "nil".to_string(),
      Value::Function(_) => "function".to_string(),
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
  fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<T>;
}

impl Interpret<Rc<Value>> for Expr {
  fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<Rc<Value>> {
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
            if let Value::Number(inner) = value.as_ref() {
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
        operator: BinaryOperator::And,
        left,
        right,
      } => {
        let left_value = left.interpret(Rc::clone(&environment))?;

        if left_value.is_truthy() {
          let right_value = right.interpret(Rc::clone(&environment))?;

          if right_value.is_truthy() {
            return Ok(right_value);
          }
        }

        Ok(Rc::new(Value::Bool(BoolValue(false))))
      }
      Expr::Binary {
        operator: BinaryOperator::Or,
        left,
        right,
      } => {
        let left_value = left.interpret(Rc::clone(&environment))?;

        if left_value.is_truthy() {
          return Ok(left_value);
        }

        let right_value = right.interpret(Rc::clone(&environment))?;

        if right_value.is_truthy() {
          Ok(right_value)
        } else {
          Ok(Rc::new(Value::Bool(BoolValue(false))))
        }
      }
      Expr::Binary {
        operator,
        left,
        right,
      } => {
        let left_value = left.interpret(Rc::clone(&environment))?;
        let right_value = right.interpret(Rc::clone(&environment))?;

        match operator {
          BinaryOperator::BangEqual => Ok(Rc::new(Value::Bool(BoolValue(
            !left_value.is_equal(&right_value)?,
          )))),
          BinaryOperator::Comma => Ok(right_value),
          BinaryOperator::EqualEqual => Ok(Rc::new(Value::Bool(BoolValue(
            left_value.is_equal(&right_value)?,
          )))),
          BinaryOperator::Plus => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Number(NumberValue(v1.0 + v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Minus => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Number(NumberValue(v1.0 - v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Star => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Number(NumberValue(v1.0 * v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Slash => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Number(NumberValue(v1.0 + v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Less => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Bool(BoolValue(v1.0 < v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::Greater => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Bool(BoolValue(v1.0 > v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::GreaterEqual => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Bool(BoolValue(v1.0 >= v2.0))))
            }
            _ => Err(anyhow!("todo")),
          },
          BinaryOperator::LessEqual => match (left_value.as_ref(), right_value.as_ref()) {
            (Value::Number(v1), Value::Number(v2)) => {
              Ok(Rc::new(Value::Bool(BoolValue(v1.0 <= v2.0))))
            }
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
        let conditional_value = conditional.interpret(Rc::clone(&environment))?;

        if conditional_value.is_truthy() {
          true_case.interpret(Rc::clone(&environment))
        } else {
          false_case.interpret(Rc::clone(&environment))
        }
      }
      Expr::Grouping { expr } => expr.interpret(environment),
      Expr::Literal { value } => match value {
        Literal::True => Ok(Value::Bool(BoolValue(true)).into()),
        Literal::False => Ok(Value::Bool(BoolValue(false)).into()),
        Literal::Number { value } => Ok(Value::Number(NumberValue(*value)).into()),
        Literal::String { value } => Ok(Value::String(StringValue(value.clone())).into()),
        Literal::Nil => Ok(Value::Nil.into()),
        Literal::Identifier { name } => environment.borrow().get(name).ok_or(
          RuntimeError::UndefinedIdentifier {
            name: name.to_string(),
          }
          .into(),
        ),
      },
      Expr::Assignment { name, expression } => {
        let value = expression.interpret(Rc::clone(&environment))?;

        environment.borrow_mut().assign(name, value)
      }
      Expr::Call {
        function,
        arguments,
      } => {
        let function_value = function.interpret(Rc::clone(&environment))?;
        let Value::Function(callable) = function_value.as_ref() else {
          todo!("err")
        };

        let mut eval_arguments: Vec<Rc<Value>> = vec![];

        for arg in arguments {
          eval_arguments.push(arg.interpret(Rc::clone(&environment))?);
        }

        Ok(callable.call(eval_arguments)?)
      }
    }
  }
}

impl Interpret<()> for Stmt {
  fn interpret(&self, environment: Rc<RefCell<Environment>>) -> Result<()> {
    match self {
      Stmt::Print { expression } => {
        let value = expression.interpret(Rc::clone(&environment))?;

        let value_str = match value.as_ref() {
          Value::Number(value) => value.0.to_string(),
          Value::String(value) => value.0.clone(),
          Value::Bool(value) => value.0.to_string(),
          Value::Nil => "nil".to_string(),
          Value::Function(_) => "function".to_string(),
        };

        println!("{}", value_str);

        Ok(())
      }
      Stmt::Block { statements } => {
        let block_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
          &environment,
        )))));

        for stmt in statements {
          stmt.interpret(Rc::clone(&block_environment))?;
        }

        Ok(())
      }
      Stmt::Expression { expression } => {
        expression.interpret(environment)?;

        Ok(())
      }
      Stmt::Declaration { name, initializer } => {
        let value = initializer.interpret(Rc::clone(&environment))?;

        environment.borrow_mut().define(name, value);

        Ok(())
      }
      Stmt::While {
        condition,
        statement,
      } => {
        while condition.interpret(Rc::clone(&environment))?.is_truthy() {
          statement.interpret(Rc::clone(&environment))?;
        }

        Ok(())
      }
      Stmt::If {
        condition,
        true_case,
        false_case,
      } => {
        if condition.interpret(Rc::clone(&environment))?.is_truthy() {
          true_case.interpret(Rc::clone(&environment))?;
        } else if let Some(statement) = false_case {
          statement.interpret(Rc::clone(&environment))?;
        }

        Ok(())
      }
    }
  }
}
