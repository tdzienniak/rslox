use crate::parser::{BinaryOperator, Expr, Literal, UnaryOperator};

pub(crate) trait Printer {
  fn print(&self) -> String;
}

impl Printer for Expr {
  fn print(&self) -> String {
    // let mut out = String::new();

    match self {
      Expr::Unary { operator, expr } => {
        let op_string = match operator {
          UnaryOperator::Bang => "!",
          UnaryOperator::Minus => "-",
        };

        format!("{}{}", op_string, expr.print())
      }
      Expr::Binary {
        operator,
        left,
        right,
      } => {
        let op_string = match operator {
          BinaryOperator::BangEqual => "!=",
          BinaryOperator::Comma => ",",
          BinaryOperator::EqualEqual => "==",
          BinaryOperator::Plus => "+",
          BinaryOperator::Minus => "-",
          BinaryOperator::Star => "*",
          BinaryOperator::Slash => "/",
          _ => "none",
        };

        let left_string = left.print();
        let right_string = right.print();

        format!("[{}]({}, {})", op_string, left_string, right_string)
      }
      Expr::Ternary {
        conditional,
        true_case,
        false_case,
      } => format!(
        "({} ? {} : {})",
        conditional.print(),
        true_case.print(),
        false_case.print()
      ),
      Expr::Grouping { expr } => expr.print(),
      Expr::Literal { value } => match value {
        Literal::True => "true".to_string(),
        Literal::False => "false".to_string(),
        Literal::Number { value } => format!("{}", value),
        Literal::String { value } => format!("\"{}\"", value),
      },
    }
  }
}
