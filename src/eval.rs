/*
 * Killjoy Lang - A simple code interpreter
 * Copyright (c) 2025 Proxy Seven
 *
 * Licensed under the MIT License.
 * You may use, copy, modify, and distribute this software freely,
 * provided that this notice is included in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
 */


use std::collections::HashMap;

use crate::{
  Expr,
  Print,
  Stmt,
  Value
};

pub struct Eval;

impl Eval {

  pub fn eval_print(&self, print_stmt: &Print, vars: &HashMap<String, Value>) {
    match print_stmt {
      Print::Ast(expr) => {
        let value: Value = self.eval_operation(expr, vars);
        println!("{:?}", value);
      },
      _ => {},
    }
  }

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