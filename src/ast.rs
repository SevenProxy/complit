#[derive(Debug)]
pub enum Expr {
  Number(i64),
  Str(String),
  Variable(String),
  Binary {
    left: Box<Expr>,
    op: String,
    right: Box<Expr>,
  },
}

#[derive(Debug)]
pub enum Stmt {
  Let(String, Expr),
  Print(Expr),
}


#[derive(Debug, Clone)]
pub enum Value {
  Number(i64),
  Str(String),
}
