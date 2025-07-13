use logos::Logos;
use std::{collections::HashMap, intrinsics::unreachable, usize};

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
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

#[derive(Debug)]
enum Expr {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}


impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    

    fn next(&mut self) -> Option(Token) {
        match self.pos < self.tokens.len() {
            true => {
                let token = self.tokens[self.pos].clone();
                self.pos += 1;

                Some(token)
            },
            false => {
                None
            },
        }
    }

    fn expect(&mut self, expected: Token) -> bool {
        match self.peek() == Some(&expected) {
            true => {
                self.next();
                true
            },
            false => {
                false
            },
        }
    }

    fn parse_number_or_var(&mut self) -> Option<Expr> {
        match self.next()? {
            Token::Number(n) => Some(Expr::Number(n)),
            Token::Identifier(name) => Some(Expr::Variable(name)),
            _ => None,
        }
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_number_or_var()?;

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Star | Token::Slash => {
                    let t = self.next().unwrap();
                    match t {
                        Token::Star => "*",
                        Token::Slash => "/",
                        _ => unreachable!(),
                    }
                }
                _ => break,
            };

            let right = self.parse_number_or_var()?;
            expr = Expr::Binary { left: Box::new(expr), op: op.to_string(), right: Box::new(right) }
        }

        Some(expr)
    }
}

fn main() {
    let mut vars = HashMap::new();

    let expr = Expr::Binary {
        left: Box::new(Expr::Number(3)),
        op: String::from("+"),
        right: Box::new(Expr::Number(4)),
    };

    let value = eval(&expr, &vars);
    vars.insert(String::from("x"), value);

    let print_expr = Expr::Variable(String::from("x"));
    println!("{}", eval(&print_expr, &vars));
}


fn eval(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) => *vars.get(name).unwrap_or(&0),
        Expr::Binary { left, op, right } => {
            let l = eval(left, vars);
            let r = eval(right, vars);
            match op.as_str() {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => l / r,
                _ => panic!("Invalid Operator!"),
            }
        }
    }
}
