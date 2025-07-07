use crate::logos::Logos;
use std::{collections::HashMap, intrinsics::unreachable, usize};


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[\t\n\f]+", logos::skip)]
    Error,

    #[token("let")]
    Let,

    #[token("print")]
    Print,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),


    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(";")]
    Semicolon,
}

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

    
}
