use std::collections::HashMap;

use crate::Expr;

pub struct Eval {
  expr: Expr,
}

impl Eval {
  fn new(self, expr: Expr) -> Self {
    Self {
      expr,
    }
  }

  fn eval_operation(&self, expr: &Expr, vars: &HashMap<String, i64>) -> Option<i64> {
    match expr {
      Expr::Number(n) => *n,
      Expr::Variable(name) => *vars.get(name).unwrap_or(&0),
      Expr::Binary { left, op, right } => {
        let l = self.eval_operation(left, vars);
        let r = self.eval_operation(right, vars);
        match op.as_str() {
          "*" => l * r,
          "/" => l / r,
          "+" => l + r,
          "-" => l + r,
          _ => panic!("Invalid Operator!"),
        }
      }
    }
  }

}