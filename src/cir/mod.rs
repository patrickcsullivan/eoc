//! CIR (C-like Intermediate Representation)

pub mod select_instr;
pub mod uncover;

use std::collections::{HashMap, HashSet};

/// Symbol used for variable names.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: String,
}

impl Symbol {
    pub fn new(value: &str) -> Symbol {
        Symbol {
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    Int(i64),
    Var(Box<Symbol>),
}

impl Arg {
    pub fn int(i: i64) -> Box<Arg> {
        Box::new(Arg::Int(i))
    }

    pub fn var(s: &str) -> Box<Arg> {
        Box::new(Arg::Var(Box::new(Symbol::new(s))))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Read,
    Arg(Box<Arg>),
    Neg(Box<Arg>),
    Add(Box<Arg>, Box<Arg>),
}

impl Expr {
    pub fn read() -> Box<Expr> {
        Box::new(Expr::Read)
    }

    pub fn arg(arg: Box<Arg>) -> Box<Expr> {
        Box::new(Expr::Arg(arg))
    }

    pub fn neg(arg: Box<Arg>) -> Box<Expr> {
        Box::new(Expr::Neg(arg))
    }

    pub fn add(op1: Box<Arg>, op2: Box<Arg>) -> Box<Expr> {
        Box::new(Expr::Add(op1, op2))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign(Box<Symbol>, Box<Expr>),
}

impl Stmt {
    pub fn assign(s: &str, expr: Box<Expr>) -> Box<Stmt> {
        Box::new(Stmt::Assign(Box::new(Symbol::new(s)), expr))
    }
}

/// A block of code.
#[derive(Clone, Debug, PartialEq)]
pub enum Tail {
    Seq(Box<Stmt>, Box<Tail>),
    Ret(Box<Expr>),
}

impl Tail {
    pub fn seq(stmt: Box<Stmt>, tail: Box<Tail>) -> Box<Tail> {
        Box::new(Tail::Seq(stmt, tail))
    }

    pub fn ret(expr: Box<Expr>) -> Box<Tail> {
        Box::new(Tail::Ret(expr))
    }
}

/// Label for a tail definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label {
    value: String,
}

impl Label {
    pub fn new(value: &str) -> Label {
        Label {
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub symbols: HashSet<Symbol>,
}

impl Info {
    pub fn new() -> Info {
        Info {
            symbols: HashSet::new(),
        }
    }
}

pub struct Program {
    pub info: Info,
    pub tails: HashMap<Label, Tail>,
}
