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


mod error;
mod parse;
mod lexer;
mod ast;
mod eval;
mod read;
mod statement;
mod functions;

pub use logos;

pub use parse::{ Parse };
pub use lexer::Token;
pub use ast::{ Expr, Value };
pub use statement::Stmt;
pub use eval::Eval;
pub use read::Read;
pub use functions::Print;