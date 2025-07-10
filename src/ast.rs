
#[derive(Debug)]
pub enum Expr {
  Number(i64),
  Variable(String),
  Binary {
    left: Box::new(Expr),
    op: String,
    right: Box::new(Expr),
  }
}