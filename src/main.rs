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

use std::{
    collections::HashMap,
    env
};

use disturbed::{
    Eval,
    Parse,
    Read,
    Stmt,
    Token,
    Value
};

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Uso: cargo run -- <namefile.kj>");
    std::process::exit(1);
  }

  let path_file: &String = &args[1];
  let mut read: Read = Read::new((path_file).to_string());
  let lexer: Result<Vec<Token>, ()> = read.file_read();

  let mut parser: Parse = Parse::new(lexer.expect("REASON"));
  let ast: Vec<Stmt> = parser.parse_all();

  let mut vars: HashMap<String, Value> = HashMap::new();

  for stmt in ast {
    match stmt {
      Stmt::Let(name, expr) => {
        let val = Eval::eval_operation(&Eval, &expr, &vars);
        vars.insert(name, val);
      }
      Stmt::Print(expr) => Eval::eval_print(&Eval, &expr, &vars),
    }
  }
}
