use super::*;
use std::collections::HashSet;

struct Ctx {
    symbols: HashSet<Symbol>,
}

impl Ctx {
    fn new() -> Ctx {
        Ctx {
            symbols: HashSet::new(),
        }
    }

    fn fold_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign(sym, _) => {
                self.symbols.insert(*sym.clone());
            }
        }
    }

    fn fold_tail(&mut self, tail: &Tail) {
        if let Tail::Seq(stmt, t) = tail {
            self.fold_stmt(stmt);
            self.fold_tail(t);
        }
    }
}

pub fn fold_program(p: Program) -> Program {
    let mut ctx = Ctx::new();
    for t in p.tails.values() {
        ctx.fold_tail(t);
    }

    Program {
        info: Info {
            symbols: ctx.symbols,
        },
        tails: p.tails,
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::fold_program;
    use std::collections::HashMap;

    #[test]
    fn basic_add_and_neg() {
        let expr = Tail::seq(
            Stmt::assign("v200000", Expr::neg(Arg::int(10))),
            Tail::ret(Expr::add(Arg::int(52), Arg::var("v200000"))),
        );
        let tails = {
            let mut tails = HashMap::new();
            tails.insert(Label::new("start"), *expr);
            tails
        };
        let program = Program {
            info: Info::new(),
            tails,
        };
        let program = fold_program(program);

        let expected_symbols = {
            let mut expected = HashSet::new();
            expected.insert(Symbol::new("v200000"));
            expected
        };
        assert_eq!(program.info.symbols, expected_symbols);
    }

    #[test]
    fn basic_add() {
        let expr = Tail::seq(
            Stmt::assign("x.1", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x.2", Expr::arg(Arg::int(22))),
                Tail::seq(
                    Stmt::assign("y", Expr::add(Arg::var("x.1"), Arg::var("x.2"))),
                    Tail::ret(Expr::arg(Arg::var("y"))),
                ),
            ),
        );
        let tails = {
            let mut tails = HashMap::new();
            tails.insert(Label::new("start"), *expr);
            tails
        };
        let program = Program {
            info: Info::new(),
            tails,
        };
        let program = fold_program(program);

        let expected_symbols = {
            let mut expected = HashSet::new();
            expected.insert(Symbol::new("x.1"));
            expected.insert(Symbol::new("x.2"));
            expected.insert(Symbol::new("y"));
            expected
        };
        assert_eq!(program.info.symbols, expected_symbols);
    }
}
