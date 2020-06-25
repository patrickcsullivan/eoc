//! RIR (R Intermediate Representation)
//! Closely corresponsds to the AST of source code.

pub mod arg_simplify;
pub mod explicate;
pub mod interp;
pub mod uniquify;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    value: String,
}

impl Symbol {
    pub fn new(value: &str) -> Symbol {
        Symbol {
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lit {
    Int(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Read,
    Lit(Lit),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Var(Box<Symbol>),
    Let(Box<Symbol>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn read() -> Box<Expr> {
        Box::new(Expr::Read)
    }

    pub fn int(i: i64) -> Box<Expr> {
        Box::new(Expr::Lit(Lit::Int(i)))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn neg(e: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Neg(e))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(e1: Box<Expr>, e2: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Add(e1, e2))
    }

    pub fn var(s: &str) -> Box<Expr> {
        Box::new(Expr::Var(Box::new(Symbol::new(s))))
    }

    pub fn let_bind(s: &str, e: Box<Expr>, body: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Let(Box::new(Symbol::new(s)), e, body))
    }
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub expr: Box<Expr>,
}

impl Program {
    pub fn new(e: Box<Expr>) -> Program {
        Program { expr: e }
    }
}

pub trait ProgramFolder {
    fn fold(&mut self, p: Program) -> Program;
}
