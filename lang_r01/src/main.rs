use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type Var = String;

#[derive(Clone, Debug)]
enum Expr {
    Read,
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Var(Var),
    Let(Var, Box<Expr>, Box<Expr>),
}

enum ROne {
    Program(Expr),
}

struct Env<'a> {
    vars: HashMap<&'a Var, &'a Expr>,
}

impl<'a> Env<'a> {
    fn new() -> Env<'a> {
        Env {
            vars: HashMap::new(),
        }
    }

    fn set(&mut self, var: &'a Var, expr: &'a Expr) {
        self.vars.insert(var, expr);
    }

    fn get(&self, var: &Var) -> Option<&'a Expr> {
        self.vars.get(var).map(|&v| v)
    }

    fn shallow_clone(&self) -> Env {
        let mut env = Env::new();
        for (&var, &expr) in self.vars.iter() {
            env.set(var, expr);
        }
        env
    }
}

fn interp_expr(expr: &Expr, env: &Env) -> i64 {
    match expr {
        Expr::Read => {
            use std::io;
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
        Expr::Var(v) => interp_expr(env.get(v).expect("undefined variable"), env),
        Expr::Let(x, e, body) => {
            let mut inner_env = env.shallow_clone();
            inner_env.set(x, e);
            interp_expr(body, &inner_env)
        }
    }
}

fn interp_r1(p: &ROne) {
    match p {
        ROne::Program(e) => println!("Result: {}", interp_expr(e, &Env::new())),
    }
}
