use super::{Expr, Program, Symbol};
use std::collections::HashMap;

struct Env {
    // vars: HashMap<&'a Symbol, &'a Expr>,
    bindings: HashMap<Box<Symbol>, i64>,
}

impl Env {
    fn new() -> Env {
        Env {
            bindings: HashMap::new(),
        }
    }

    fn set(&mut self, sym: Box<Symbol>, val: i64) {
        self.bindings.insert(sym, val);
    }

    fn get(&self, sym: &Symbol) -> Option<i64> {
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

fn interp_expr(expr: &Expr, env: &Env) -> i64 {
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
            input.parse().expect("expected integer input")
        }
        Expr::Int(i) => *i,
        Expr::Neg(e) => -interp_expr(e, env),
        Expr::Add(e1, e2) => interp_expr(e1, env) + interp_expr(e2, env),
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
