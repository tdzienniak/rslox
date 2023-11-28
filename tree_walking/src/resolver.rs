use crate::parser::{Expr, Literal, Stmt};
use std::collections::HashMap;

type Scope = HashMap<String, bool>;
pub(crate) type Locals = HashMap<usize, usize>;

pub(crate) struct Resolver {
  scopes: Vec<Scope>,
  locals: Locals,
}

impl Resolver {
  pub(crate) fn new() -> Self {
    Resolver {
      scopes: vec![HashMap::from([("println".to_string(), true), ("clock".to_string(), true)]), HashMap::new()],
      locals: HashMap::new(),
    }
  }

  pub(crate) fn resolve_program(mut self, program: &[Stmt]) -> Locals {
    for stmt in program {
      self.resolve_stmt(stmt);
    }

    self.locals
  }

  fn resolve_expr(&mut self, expr: &Expr) {
    match expr {
      Expr::Ternary {
        conditional,
        true_case,
        false_case,
      } => {
        self.resolve_expr(conditional);
        self.resolve_expr(true_case);
        self.resolve_expr(false_case);
      }
      Expr::Binary { left, right, .. } => {
        self.resolve_expr(left);
        self.resolve_expr(right);
      }
      Expr::Unary { expr, .. } => {
        self.resolve_expr(expr);
      }
      Expr::Grouping { expr } => {
        self.resolve_expr(expr);
      }
      Expr::Literal { value } => {
        if let Literal::Identifier { name, id } = value {
          if let Some(scope) = self.scopes.last() {
            if Some(&false) == scope.get(name) {
              // TODO: report error: "Can't read local variable in its own initializer."
            }
          }

          self.resolve_local(name, id);
        }
      }
      Expr::Assignment {
        name,
        expression,
        id,
      } => {
        self.resolve_expr(expression);
        self.resolve_local(name, id);
      }
      Expr::Call { arguments, function } => {
        self.resolve_expr(function);

        for arg in arguments {
          self.resolve_expr(arg);
        }
      }
    }
  }

  fn resolve_stmt(&mut self, stmt: &Stmt) {
    match stmt {
      Stmt::Expression { expression } => {
        self.resolve_expr(expression);
      }
      Stmt::Declaration { name, initializer } => {
        self.declare(name);

        self.resolve_expr(initializer);

        self.define(name);
      }
      Stmt::FunDeclaration {
        name,
        body,
        parameters,
      } => {
        self.declare(name);
        self.define(name);

        self.begin_scope();
        for param in parameters {
          self.declare(param);
          self.define(param);
        }

        for stmt in body {
          self.resolve_stmt(stmt);
        }

        self.end_scope();
      }
      Stmt::Block { statements } => {
        self.begin_scope();

        for stmt in statements {
          self.resolve_stmt(stmt);
        }

        self.end_scope();
      }
      Stmt::While {
        statement,
        condition,
      } => {
        self.resolve_expr(condition);
        self.resolve_stmt(statement)
      }
      Stmt::If {
        condition,
        true_case,
        false_case,
      } => {
        self.resolve_expr(condition);
        self.resolve_stmt(true_case);
        if let Some(stmt) = false_case {
          self.resolve_stmt(stmt);
        }
      }
    }
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.to_string(), false);
    }
  }

  fn define(&mut self, name: &str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.to_string(), true);
    }
  }
  fn resolve_local(&mut self, name: &str, expr_id: &usize) {
    println!("{:?}", self.scopes);
    for (distance_from_last, scope) in self.scopes.iter().rev().enumerate() {
      if let Some(&true) = scope.get(name) {
        self.locals.insert(*expr_id, distance_from_last);

        return;
      }
    }

    panic!("variable {} must be defined before it's used", name);
  }
}
