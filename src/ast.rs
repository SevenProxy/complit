#[derive(Debug)]
pub enum Expr {
  Number(i64),
  Variable(String),
  Binary {
    left: Box<Expr>,
    op: String,
    right: Box<Expr>,
  },
}