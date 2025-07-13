use crate::logos::Logos;
use std::usize;


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[\t\n\f]+", logos::skip)]
    Error,

    #[token("let")]
    Let,

    #[token("print")]
    Print,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

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
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(";")]
    Semicolon,
}

