#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    Value: String,
}

impl Symbol {
    pub fn new(value: String) -> Symbol {
        Symbol { Value: value }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Read,
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Var(Box<Symbol>),
    Let(Box<Symbol>, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub expr: Box<Expr>,
}

pub trait ExprFolder {
    fn fold(&mut self, e: Box<Expr>) -> Box<Expr> {
        match *e {
            Expr::Neg(e) => self.fold_neg(e),
            Expr::Add(e1, e2) => self.fold_add(e1, e2),
            Expr::Var(s) => self.fold_var(s),
            Expr::Let(sym, e, body) => self.fold_let(sym, e, body),
            _ => e, // By default leaf expressions just return identity.
        }
    }

    fn fold_sym(&mut self, s: Box<Symbol>) -> Box<Symbol> {
        s
    }

    fn fold_neg(&mut self, e: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Neg(self.fold(e)))
    }

    fn fold_add(&mut self, e1: Box<Expr>, e2: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Add(self.fold(e1), self.fold(e2)))
    }

    fn fold_var(&mut self, s: Box<Symbol>) -> Box<Expr> {
        Box::new(Expr::Var(self.fold_sym(s)))
    }

    fn fold_let(&mut self, sym: Box<Symbol>, e: Box<Expr>, body: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Let(self.fold_sym(sym), self.fold(e), self.fold(body)))
    }

    fn fold_int(&mut self, i: i64) -> Box<Expr> {
        Box::new(Expr::Int(i))
    }
}

pub trait ProgramFolder {
    fn fold(&mut self, p: Program) -> Program;
}
