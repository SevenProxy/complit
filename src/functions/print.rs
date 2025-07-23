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


use crate::{
  Expr,
};

#[derive(Debug)]
pub enum Print {
  Variable(String),
  Ast(Expr),
}