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
  Print,
  Stmt,
  Token
};
use std::usize;

pub struct Parse {
  tokens: Vec<Token>,
  pos: usize,
  line: u32,
}

impl Parse {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      pos: 0,
      line: 0,
    }
  }

  pub fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.pos)
  }

  fn next_token(&mut self) -> Option<Token> {
    match self.pos < self.tokens.len() {
      true => {
        let tokens = self.tokens[self.pos].clone();
        self.pos += 1;
        Some(tokens)
      },
      false => None,
    }
  }

  fn parse_primary(&mut self) -> Option<Expr> {
    match self.next_token()? {
      Token::Identifier(name) => Some(Expr::Variable(name)),
      Token::Number(n) => Some(Expr::Number(n)),
      Token::Str(s) => Some(Expr::Str(s)),
      Token::LParen => {
        let expr: Expr = self.parse_operation()?;
        match self.next_token()? {
          Token::RParen => Some(expr),
          _ => None,
        }
      }
      _ => None,
    }
  }

  fn parse_term(&mut self) -> Option<Expr> {
    let mut expr: Export = self.parse_factory();

    /*
      tok: &Token
     */
    while let Some(tok) = self.peek() {
      let operation = match tok {
        
      }
    }

    Some(expr)
  }

  fn parse_factory(&mut self) -> Option<Expr> {
    let mut expr = self.parse_primary()?;
    
    while let Some(tok) = self.peek() {
      let operation = match tok {
        Token::Slash => "/",
        Token::Star => "*",
        Token::Plus => "+",
        Token::Minus => "-",
        _ => break,
      };

    }
    Some(expr)
  }
    
  fn parse_stmt(&mut self) -> Option<Stmt> {
    match self.peek()? {
      Token::Line => {
        self.next_token();
        self.line += 1;
        return self.parse_stmt();
      },
      Token::Let => {
        self.next_token();
        if let Some(Token::Identifier(name)) = self.next_token() {
          if let Some(Token::Equal) = self.next_token() {
            let expr: Expr = self.parse_operation()?;
            if let Some(Token::Semicolon) = self.next_token() {
              return Some(Stmt::Let(name, expr));
            } else {
              eprintln!("[Error] - Invalid Sintaxe! Please add ; in line end.");
              std::process::exit(1);
            }
          }
        }
      },
      Token::Print => {
        self.next_token();
        let expr: Expr = self.parse_operation()?;
        if let Some(Token::Semicolon) = self.next_token() {
          let print_function: Print = Print::Ast(expr);
          return Some(Stmt::Print(print_function));
        }
      },
      _ => {},
    }

    None
  }

  pub fn parse_all(&mut self) -> Vec<Stmt> {
    let mut stmts: Vec<Stmt> = Vec::new();
    while self.peek().is_some() {
      if let Some(stmt) = self.parse_stmt() {
        stmts.push(stmt);
      } else {
        println!("[Error] - Interpretation Failed.");
        println!("Line: {}", self.line + 1);
        
        if let Some(token) = self.peek() {
          println!("Unexpected token: {:?}", token);
        } else {
          println!("Unexpected end of input.");
        }

        break;
      }
    }
    stmts
  }

  pub fn expect(&mut self, expected: Token) -> bool {
    match self.peek() == Some(&expected) {
      true => {
        self.next_token();
        true
      },
      false => false,
    }
  }

}
