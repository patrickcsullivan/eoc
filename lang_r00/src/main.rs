use std::boxed::Box;

fn main() {
    // let p1 = RZero::Program(Expr::Add(
    //     Box::new(Expr::Read),
    //     Box::new(Expr::Neg(Box::new(Expr::Int(8)))),
    // ));
    // interp_r0(&p1);

    let expr = Expr::Add(
        Box::new(Expr::Int(20)),
        Box::new(Expr::Neg(Box::new(Expr::Int(8)))),
    );
    interp_r0(&RZero::Program(expr.clone()));
    interp_r0(&RZero::Program(optimize_expr(&expr)));
}

#[derive(Clone, Debug)]
enum Expr {
    Read,
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

enum RZero {
    Program(Expr),
}

fn interp_expr(e: &Expr) -> i64 {
    match e {
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
        Expr::Neg(e) => -interp_expr(e),
        Expr::Add(e1, e2) => interp_expr(e1) + interp_expr(e2),
    }
}

fn interp_r0(p: &RZero) {
    match p {
        RZero::Program(e) => println!("Result: {}", interp_expr(e)),
    }
}

fn optimize_expr(e: &Expr) -> Expr {
    match e {
        Expr::Neg(e) => pe_negate(&optimize_expr(e)),
        Expr::Add(e1, e2) => pe_add(&optimize_expr(e1), &optimize_expr(e2)),
        _ => e.clone(),
    }
}

fn pe_negate(e: &Expr) -> Expr {
    match e {
        Expr::Int(i) => Expr::Int(-i),
        _ => Expr::Neg(Box::new(e.clone())),
    }
}

fn pe_add(e1: &Expr, e2: &Expr) -> Expr {
    match (e1, e2) {
        (Expr::Int(i1), Expr::Int(i2)) => Expr::Int(i1 + i2),
        _ => Expr::Add(Box::new(e1.clone()), Box::new(e2.clone())),
    }
}
