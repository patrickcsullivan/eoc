mod cir;
mod pxir;
mod rir;

fn main() {
    use rir::{Expr, ExprFolder, Program};
    let expr = Expr::let_bind(
        "my_var",
        Expr::int(42),
        Expr::let_bind(
            "input",
            Expr::read(),
            Expr::let_bind(
                "my_var",
                Expr::add(Expr::var("my_var"), Expr::neg(Expr::var("input"))),
                Expr::var("my_var"),
            ),
        ),
    );

    let mut uniquify_ctx = rir::uniquify::ExprUniquifier::new();
    let expr = uniquify_ctx.fold(expr);

    let mut arg_simplify_ctx = rir::arg_simplify::ExprArgSimplifier::new(uniquify_ctx.counter);
    let expr = arg_simplify_ctx.fold(expr);

    let prog = Program::new(expr);
    // rir::interp::interp(&prog);

    let prog = rir::explicate::fold_program(prog);
    let prog = cir::uncover::fold_program(prog);
    for s in prog.info.symbols {
        println!("LOCAL: {}", s.value);
    }
}
