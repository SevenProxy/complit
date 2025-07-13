use crate::{ Token, Expr };
use std::usize;

pub struct Parse {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parse {
    
    fn new(self, tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.pos > self.tokens.len() {
            true => {
                let tokens = self.tokens[self.pos].clone();
                self.pos += 1;

                Some(tokens)
            },
            false => None,
        }
    }

    fn parse_number_or_var(&mut self) -> Option<Expr> {
        match self.next_token()? {
            Token::Number(n) => Some(Expr::Number(n)),
            Token::Identifier(name) => Some(Expr::Variable(name)),
            _ => None,
        }
    }

    fn parse_terms(&mut self) -> Option<Expr> {
        let mut expr = self.parse_number_or_var()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star | Token::Slash => {
                    let t = self.next_token().unwrap();
                    match t {
                        Token::Star => "*",
                        Token::Slash => "/",
                        _ => unreachable!(),
                    }
                },
                _ => break,
            };

            let right = self.parse_number_or_var()?;
            expr = Expr::Binary { left: Box::new(expr), op: op.to_string(), right: Box::new(right) };
        }
        
        Some(expr)
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
