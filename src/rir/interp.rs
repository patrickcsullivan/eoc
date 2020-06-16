use super::{Expr, Literal, Program, Symbol};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

struct Env {
    // vars: HashMap<&'a Symbol, &'a Expr>,
    bindings: HashMap<Box<Symbol>, Literal>,
}

impl Env {
    fn new() -> Env {
        Env {
            bindings: HashMap::new(),
        }
    }

    fn set(&mut self, sym: Box<Symbol>, val: Literal) {
        self.bindings.insert(sym, val);
    }

    fn get(&self, sym: &Symbol) -> Option<Literal> {
        self.bindings.get(sym).copied()
    }

    fn shallow_clone(&self) -> Env {
        let mut env = Env::new();
        for (sym, val) in self.bindings.iter() {
            env.set(sym.clone(), val.clone());
        }
        env
    }
}

fn interp_expr(expr: &Expr, env: &Env) -> Literal {
    match expr {
        Expr::Read => {
            use std::io;
            use std::io::prelude::*;
            print!("Provide input: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error reading input");
            input = input.trim().to_string();
            TryFrom::try_from(input).expect("could not parse input")
        }
        Expr::Lit(lit) => *lit,
        Expr::Neg(e) => match interp_expr(e, env) {
            Literal::Int(i) => Literal::Int(-i),
        },
        Expr::Add(e1, e2) => {
            let ipterpd1 = interp_expr(e1, env);
            let interpd2 = interp_expr(e2, env);
            match (ipterpd1, interpd2) {
                (Literal::Int(i1), Literal::Int(i2)) => Literal::Int(i1 + i2),
            }
        }
        Expr::Var(sym) => env.get(sym).expect("undefined variable"),
        Expr::Let(sym, e, body) => {
            let val = interp_expr(e, env);
            let mut new_env = env.shallow_clone();
            new_env.set(sym.clone(), val);
            interp_expr(body, &new_env)
        }
    }
}

pub fn interp(p: &Program) {
    println!("Result: {}", interp_expr(&p.expr, &Env::new()))
}

impl TryFrom<String> for Literal {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let x = value.parse()?;
        Ok(Literal::Int(x))
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Literal::Int(i) => write!(f, "{}", i),
        }
    }
}
