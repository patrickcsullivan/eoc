mod cir;
mod driver;
mod pxir;
mod rir;

use driver::drive;
use rir::Expr;

fn main() {
    use rir::Expr;
    // let expr = Expr::let_bind(
    //     "my_var",
    //     Expr::int(42),
    //     Expr::let_bind(
    //         "input",
    //         Expr::read(),
    //         Expr::let_bind(
    //             "my_var",
    //             Expr::add(Expr::var("my_var"), Expr::neg(Expr::var("input"))),
    //             Expr::var("my_var"),
    //         ),
    //     ),
    // );
    let expr = Expr::add(Expr::int(52), Expr::neg(Expr::int(10)));
    let out = drive(*expr);
    print!("OUT:\n{}", out);
}
