use std::collections::HashMap;

use crate::{
  Expr,
  Value,
};

pub struct Eval;

impl Eval {

  pub fn eval_operation(&self, expr: &Expr, vars: &HashMap<String, Value>) -> Value {
    match expr {
      Expr::Number(n) => Value::Number(*n),
      Expr::Str(s) => Value::Str(s.clone()),
      Expr::Variable(name) => vars.get(name).cloned().unwrap_or(Value::Number(0)),
      Expr::Binary { left, op, right } => {
        let l: Value = self.eval_operation(left, vars);
        let r: Value = self.eval_operation(right, vars);
        match (l, r, op.as_str()) {
          (Value::Number(x), Value::Number(y), "*") => Value::Number(x * y),
          (Value::Number(x), Value::Number(y), "/") => Value::Number(x / y),
          (Value::Number(x), Value::Number(y), "+") => Value::Number(x + y),
          (Value::Number(x), Value::Number(y), "-") => Value::Number(x - y),

          (Value::Str(x), Value::Str(y), "+") => Value::Str(x + &y),

          _ => panic!("Invalid Operator!"),
        }
      }
    }
  }

}