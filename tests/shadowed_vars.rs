use eoc::driver::drive;
use eoc::rir::Expr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn shadowed_vars() -> std::io::Result<()> {
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
    let out = drive(*expr);

    let path = Path::new("./tests/target/shadowed_vars.s");
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;
    file.write_all(out.as_bytes())
}

// Link compiled code with runtime by running
// gcc -g ./runtime/runtime.o ./target/shadowed_vars.s -o ./target/shadowed_vars.o
