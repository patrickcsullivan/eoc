mod ast;
mod interp;
mod uniquify;
mod x86;

fn main() {
    use ast::{Expr, Program};
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
    let prog = Program::new(expr);

    // use ast::ProgramFolder;
    // let mut ctx = uniquify::ProgramUniquifier {};
    // let prog2 = ctx.fold(prog);

    interp::interp(&prog);
}

// struct Ctx {
//     counter: u64,
// }

// impl Ctx {
//     pub fn new() -> Ctx {
//         Ctx { counter: 0 }
//     }

//     pub fn get_counter(&mut self) -> u64 {
//         let c = self.counter;
//         self.counter += 1;
//         c
//     }

//     pub fn perform_calculation(&mut self, my_num: u64) -> u64 {
//         self.get_counter() + my_num
//     }
// }

// pub fn perform_calculation(ctx: &mut Ctx, my_num: u64) -> u64 {
//     ctx.get_counter() + my_num
// }
