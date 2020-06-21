use eoc::driver::drive;
use eoc::rir::Expr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn nested_let_assigns() -> std::io::Result<()> {
    let expr = Expr::let_bind(
        "y",
        Expr::let_bind(
            "x.1",
            Expr::int(20),
            Expr::let_bind(
                "x.2",
                Expr::int(22),
                Expr::add(Expr::var("x.1"), Expr::var("x.2")),
            ),
        ),
        Expr::var("y"),
    );
    let out = drive(*expr);

    let path = Path::new("./tests/target/nestet_let_assigns.s");
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;
    file.write_all(out.as_bytes())
}

// Link compiled code with runtime by running
// gcc -g ./runtime/runtime.o ./target/nested_let_assigns.s -o ./target/nested_let_assigns.o
