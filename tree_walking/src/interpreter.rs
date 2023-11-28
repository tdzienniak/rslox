use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::parser::{BinaryOperator, Expr, Literal, Stmt, UnaryOperator};
use crate::resolver::Locals;
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(crate) struct NumberValue(f64);

#[derive(Debug)]
pub(crate) struct StringValue(String);

#[derive(Debug)]
pub(crate) struct BoolValue(bool);

pub(crate) trait Callable {
  fn call(&self, arguments: Vec<Rc<Value>>, interpreter: &mut Interpreter) -> Result<Rc<Value>>;
}

pub(crate) struct NativeClock;

impl Callable for NativeClock {
  fn call(&self, _arguments: Vec<Rc<Value>>, interpreter: &mut Interpreter) -> Result<Rc<Value>> {
    let start = SystemTime::now();
    let since_the_epoch = start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
    Ok(Rc::new(Value::Number(NumberValue(
      since_the_epoch.as_secs_f64(),
    ))))
  }
}

pub(crate) struct NativePrintln;

impl Callable for NativePrintln {
  fn call(&self, arguments: Vec<Rc<Value>>, interpreter: &mut Interpreter) -> Result<Rc<Value>> {
    println!(
      "{}",
      arguments
        .iter()
        .map(|value| format!("{}", value))
        .collect::<Vec<String>>()
        .join(" ")
    );

    Ok(Rc::new(Value::Nil))
  }
}

pub(crate) struct Fun {
  parameters: Vec<String>,
  body: Vec<Stmt>,
  name: String,
  environment: Rc<RefCell<Environment>>,
}

impl Fun {
  fn new(parameters: Vec<String>, body: Vec<Stmt>, name: String, environment: Environment) -> Self {
    Fun {
      body,
      parameters,
      name,
      environment: Rc::new(RefCell::new(environment)),
    }
  }
}

impl Callable for Fun {
  fn call(&self, arguments: Vec<Rc<Value>>, interpreter: &mut Interpreter) -> Result<Rc<Value>> {
    if arguments.len() != self.parameters.len() {
      panic!("aaaaaa")
    }

    for (index, param) in self.parameters.iter().enumerate() {
      self
        .environment
        .borrow_mut()
        .define(param, Rc::clone(&arguments[index]));
    }

    for stmt in &self.body {
      interpreter.interpret_stmt(stmt, Rc::clone(&self.environment))?;
    }

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

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let value_as_string = match self {
      Value::Number(value) => value.0.to_string(),
      Value::String(value) => value.0.clone(),
      Value::Bool(value) => value.0.to_string(),
      Value::Nil => "nil".to_string(),
      Value::Function(_) => "function".to_string(),
    };

    write!(f, "{}", value_as_string)
  }
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

pub(crate) struct Interpreter {
  pub(crate) locals: Locals,
}

impl Interpreter {
  pub(crate) fn new(locals: Locals) -> Self {
    Interpreter { locals }
  }

  pub(crate) fn interpret_program(mut self, program: Vec<Stmt>) -> Result<()> {
    let global = Rc::new(RefCell::new(Environment::new(None)));

    {
      let mut env = global.borrow_mut();

      env.define("clock", Rc::new(Value::Function(Box::new(NativeClock {}))));
      env.define(
        "println",
        Rc::new(Value::Function(Box::new(NativePrintln {}))),
      );
    }

    let top = Rc::new(RefCell::new(Environment::new(Some(global))));

    for stmt in &program {
      self.interpret_stmt(stmt, Rc::clone(&top))?;
    }

    Ok(())
  }

  fn interpret_expr(
    &mut self,
    expr: &Expr,
    environment: Rc<RefCell<Environment>>,
  ) -> Result<Rc<Value>> {
    match expr {
      Expr::Unary { operator, expr } => {
        let value = self.interpret_expr(expr, environment)?;
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
        let left_value = self.interpret_expr(left, Rc::clone(&environment))?;

        if left_value.is_truthy() {
          let right_value = self.interpret_expr(right, Rc::clone(&environment))?;

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
        let left_value = self.interpret_expr(left, Rc::clone(&environment))?;

        if left_value.is_truthy() {
          return Ok(left_value);
        }

        let right_value = self.interpret_expr(right, Rc::clone(&environment))?;

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
        let left_value = self.interpret_expr(left, Rc::clone(&environment))?;
        let right_value = self.interpret_expr(right, Rc::clone(&environment))?;

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
        let conditional_value = self.interpret_expr(conditional, Rc::clone(&environment))?;

        if conditional_value.is_truthy() {
          self.interpret_expr(true_case, Rc::clone(&environment))
        } else {
          self.interpret_expr(false_case, Rc::clone(&environment))
        }
      }
      Expr::Grouping { expr } => self.interpret_expr(expr, environment),
      Expr::Literal { value } => match value {
        Literal::True => Ok(Value::Bool(BoolValue(true)).into()),
        Literal::False => Ok(Value::Bool(BoolValue(false)).into()),
        Literal::Number { value } => Ok(Value::Number(NumberValue(*value)).into()),
        Literal::String { value } => Ok(Value::String(StringValue(value.clone())).into()),
        Literal::Nil => Ok(Value::Nil.into()),
        Literal::Identifier { name, id } => environment
          .borrow()
          .get(name, *self.locals.get(id).unwrap())
          .ok_or(
            RuntimeError::UndefinedIdentifier {
              name: name.to_string(),
            }
            .into(),
          ),
      },
      Expr::Assignment {
        name,
        expression,
        id,
      } => {
        let value = self.interpret_expr(expression, Rc::clone(&environment))?;

        Ok(
          environment
            .borrow_mut()
            .assign(name, value, *self.locals.get(id).unwrap()),
        )
      }
      Expr::Call {
        function,
        arguments,
      } => {
        let function_value = self.interpret_expr(function, Rc::clone(&environment))?;
        let Value::Function(callable) = function_value.as_ref() else {
          todo!("err")
        };

        let mut eval_arguments: Vec<Rc<Value>> = vec![];

        for arg in arguments {
          eval_arguments.push(self.interpret_expr(arg, Rc::clone(&environment))?);
        }

        Ok(callable.call(eval_arguments, self)?)
      }
    }
  }

  fn interpret_stmt(&mut self, stmt: &Stmt, environment: Rc<RefCell<Environment>>) -> Result<()> {
    match stmt {
      Stmt::Block { statements } => {
        let block_environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
          &environment,
        )))));

        for stmt in statements {
          self.interpret_stmt(stmt, Rc::clone(&block_environment))?;
        }
      }
      Stmt::Expression { expression } => {
        self.interpret_expr(expression, environment)?;
      }
      Stmt::Declaration { name, initializer } => {
        let value = self.interpret_expr(initializer, Rc::clone(&environment))?;

        environment.borrow_mut().define(name, value);
      }
      Stmt::FunDeclaration {
        name,
        parameters,
        body,
      } => {
        let value = Fun::new(
          parameters.clone(),
          body.clone(),
          name.clone(),
          Environment::new(Some(Rc::clone(&environment))),
        );

        environment
          .borrow_mut()
          .define(name, Rc::new(Value::Function(Box::new(value))));
      }
      Stmt::While {
        condition,
        statement,
      } => {
        while self
          .interpret_expr(condition, Rc::clone(&environment))?
          .is_truthy()
        {
          self.interpret_stmt(statement, Rc::clone(&environment))?;
        }
      }
      Stmt::If {
        condition,
        true_case,
        false_case,
      } => {
        if self
          .interpret_expr(condition, Rc::clone(&environment))?
          .is_truthy()
        {
          self.interpret_stmt(true_case, Rc::clone(&environment))?;
        } else if let Some(statement) = false_case {
          self.interpret_stmt(statement, Rc::clone(&environment))?;
        }
      }
    };

    Ok(())
  }
}
