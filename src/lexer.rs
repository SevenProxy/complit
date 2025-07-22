use crate::logos::Logos;
use std::usize;


#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[\t\n\f]+", logos::skip)]
    Ignored,

    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    #[token("armazena_robozinho")]
    Let,

    #[token("mostra_na_tela_robozinho")]
    Print,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice: &'s str = lex.slice();
        Some(slice[1..slice.len() - 1].to_string())
    })]
    Str(String),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),

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

