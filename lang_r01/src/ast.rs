pub type Var = String;

#[derive(Clone, Debug)]
pub enum Expr {
    Read,
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Var(Var),
    Let(Var, Box<Expr>, Box<Expr>),
}

pub struct Program {
    pub Expr: Expr,
}
