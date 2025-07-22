use crate::{ Token, Expr, Stmt };
use std::usize;

pub struct Parse {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parse {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
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
            Token::Number(n) => Some(Expr::Number(n)),
            Token::Identifier(name) => Some(Expr::Variable(name)),
            Token::Str(s) => Some(Expr::Str(s)), 
            Token::LParen => {
                let expr = self.parse_expr()?;
                match self.next_token()? {
                    Token::RParen => Some(expr),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star => "*",
                Token::Slash => "/",
                _ => break,
            };

            self.next_token();

            let right = self.parse_primary()?;
            expr = Expr::Binary { left: Box::new(expr), op: op.to_string(), right: Box::new(right) };
        }
        
        Some(expr)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };

            self.next_token();
            let rhs = self.parse_term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: op.to_string(),
                right: Box::new(rhs),
            };
        }

        Some(expr)
    }
    
    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek()? {
            Token::Let => {
                self.next_token();
                if let Some(Token::Identifier(name)) = self.next_token() {
                    if let Some(Token::Equal) = self.next_token() {
                        let expr = self.parse_expr()?;
                        if let Some(Token::Semicolon) = self.next_token() {
                            return Some(Stmt::Let(name, expr));
                        }
                    }
                }
            },
            Token::Print => {
                self.next_token();
                let expr = self.parse_expr()?;
                if let Some(Token::Semicolon) = self.next_token() {
                    return Some(Stmt::Print(expr));
                }
            },
            _ => {},
        }

        None
    }

    pub fn parse_all(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                println!("Error de parse");
                break;
            }
        }
        stmts
    }

    fn expect(&mut self, expected: Token) -> bool {
        match self.peek() == Some(&expected) {
            true => {
                self.next_token();
                true
            },
            false => false,
        }
    }

}
