use crate::logos::{ Logos, Token };
use std::{collections::HashMap, intrinsics::unreachable, usize};

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
        match self.tokens.get(self.pos) {
            Some(token) => Some(token),
            None => None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.pos? {
            true => {
                let tokens = self.tokens[self.pos].clone();
                self.post += 1;

                Some(tokens)
            },
            false => None,
        }
    }

    fn parse_number(&self, token: Token) -> Option<Token> {
        match self.next() {
            
        }
    }
}
