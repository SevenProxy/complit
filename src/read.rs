use std::fs;

use logos::Logos;

use crate::Token;

pub struct Read {
  pub file_name: String,
}

impl Read {

  pub fn new(file_name: String) -> Self {
    Self {
      file_name,
    }
  }

  pub fn file_read(&mut self) -> Result<Vec<Token>, ()> {
    let message_warning_not_found_file: String = format!("Não foi possível ler o arquivo {}", self.file_name);
    let content: String = fs::read_to_string(&self.file_name)
      .expect(&message_warning_not_found_file);

    println!("{:?}", content);
    if content.trim().is_empty() {
      return Err(());
    }

    let lexer: Vec<Token> = Token::lexer(&content)
      .filter_map(Result::ok)
      .collect::<Vec<_>>();

    Ok(lexer)
  }
}