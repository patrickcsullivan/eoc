use super::super::cir;
use super::{Expr, Lit, Program};
use std::collections::HashMap;

fn prepend_expr_to_tail(
    expr: Box<cir::Expr>,
    assign_to_with_tail: Option<(&str, Box<cir::Tail>)>,
) -> Box<cir::Tail> {
    match assign_to_with_tail {
        None => cir::Tail::ret(expr),
        Some((assign_to, tail)) => cir::Tail::seq(cir::Stmt::assign(assign_to, expr), tail),
    }
}

fn fold_op(expr: Expr) -> Box<cir::Arg> {
    match expr {
        Expr::Lit(Lit::Int(i)) => cir::Arg::int(i),
        Expr::Var(sym) => cir::Arg::var(&sym.value),
        _ => panic!("uniquify pass should have converted all operands into vars or lits"),
    }
}

fn fold_let_assign(assign_to: &str, expr: Expr, tail: Box<cir::Tail>) -> Box<cir::Tail> {
    match expr {
        Expr::Read => {
            let assign_val = cir::Expr::read();
            cir::Tail::seq(cir::Stmt::assign(assign_to, assign_val), tail)
        }
        Expr::Lit(Lit::Int(i)) => {
            let assign_val = cir::Expr::arg(cir::Arg::int(i));
            cir::Tail::seq(cir::Stmt::assign(assign_to, assign_val), tail)
        }
        Expr::Neg(op) => {
            let assign_val = cir::Expr::neg(fold_op(*op));
            cir::Tail::seq(cir::Stmt::assign(assign_to, assign_val), tail)
        }
        Expr::Add(op1, op2) => {
            let assign_val = cir::Expr::add(fold_op(*op1), fold_op(*op2));
            cir::Tail::seq(cir::Stmt::assign(assign_to, assign_val), tail)
        }
        Expr::Var(sym) => {
            let assign_val = cir::Expr::arg(cir::Arg::var(&sym.value));
            cir::Tail::seq(cir::Stmt::assign(assign_to, assign_val), tail)
        }
        Expr::Let(sym, assn, body) => {
            let tail_with_parent_assn = fold_let_body(*body, Some((assign_to, tail)));
            fold_let_assign(&sym.value, *assn, tail_with_parent_assn)
        }
    }
}

fn fold_let_body(
    expr: Expr,
    assign_to_with_tail: Option<(&str, Box<cir::Tail>)>,
) -> Box<cir::Tail> {
    match expr {
        Expr::Read => {
            let c_expr = cir::Expr::read();
            prepend_expr_to_tail(c_expr, assign_to_with_tail)
        }
        Expr::Lit(Lit::Int(i)) => {
            let c_expr = cir::Expr::arg(cir::Arg::int(i));
            prepend_expr_to_tail(c_expr, assign_to_with_tail)
        }
        Expr::Neg(op) => {
            let c_expr = cir::Expr::neg(fold_op(*op));
            prepend_expr_to_tail(c_expr, assign_to_with_tail)
        }
        Expr::Add(op1, op2) => {
            let c_expr = cir::Expr::add(fold_op(*op1), fold_op(*op2));
            prepend_expr_to_tail(c_expr, assign_to_with_tail)
        }
        Expr::Var(sym) => {
            let c_expr = cir::Expr::arg(cir::Arg::var(&sym.value));
            prepend_expr_to_tail(c_expr, assign_to_with_tail)
        }
        Expr::Let(sym, assn, body) => {
            let tail = fold_let_body(*body, assign_to_with_tail);
            fold_let_assign(&sym.value, *assn, tail)
        }
    }
}

fn fold_root_expr(expr: Expr) -> Box<cir::Tail> {
    match expr {
        Expr::Read => cir::Tail::ret(cir::Expr::read()),
        Expr::Lit(Lit::Int(i)) => cir::Tail::ret(cir::Expr::arg(cir::Arg::int(i))),
        Expr::Neg(op) => cir::Tail::ret(cir::Expr::neg(fold_op(*op))),
        Expr::Add(op1, op2) => cir::Tail::ret(cir::Expr::add(fold_op(*op1), fold_op(*op2))),
        Expr::Var(sym) => cir::Tail::ret(cir::Expr::arg(cir::Arg::var(&sym.value))),
        Expr::Let(sym, assn, body) => {
            let tail = fold_let_body(*body, None);
            fold_let_assign(&sym.value, *assn, tail)
        }
    }
}

pub fn fold_program(p: Program) -> cir::Program {
    let start_proc = fold_root_expr(*p.expr);
    let tails = {
        let mut tails = HashMap::new();
        tails.insert(cir::Label::new("start"), *start_proc);
        tails
    };
    cir::Program {
        info: cir::Info::default(),
        tails,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::cir;
    use super::super::Expr;
    use super::fold_root_expr;

    #[test]
    fn basic_add_and_neg() {
        let expr = Expr::let_bind(
            "v200000",
            Expr::neg(Expr::int(10)),
            Expr::add(Expr::int(52), Expr::var("v200000")),
        );
        let expected = cir::Tail::seq(
            cir::Stmt::assign("v200000", cir::Expr::neg(cir::Arg::int(10))),
            cir::Tail::ret(cir::Expr::add(cir::Arg::int(52), cir::Arg::var("v200000"))),
        );

        let actual = fold_root_expr(*expr);
        assert_eq!(actual, expected);
    }

    #[test]
    fn nested_let_assigns() {
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
        let expected = cir::Tail::seq(
            cir::Stmt::assign("x.1", cir::Expr::arg(cir::Arg::int(20))),
            cir::Tail::seq(
                cir::Stmt::assign("x.2", cir::Expr::arg(cir::Arg::int(22))),
                cir::Tail::seq(
                    cir::Stmt::assign(
                        "y",
                        cir::Expr::add(cir::Arg::var("x.1"), cir::Arg::var("x.2")),
                    ),
                    cir::Tail::ret(cir::Expr::arg(cir::Arg::var("y"))),
                ),
            ),
        );

        let actual = fold_root_expr(*expr);
        assert_eq!(actual, expected);
    }

    #[test]
    fn let_assign_and_body() {
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

        let expected = cir::Tail::seq(
            cir::Stmt::assign("x.1", cir::Expr::arg(cir::Arg::int(20))),
            cir::Tail::seq(
                cir::Stmt::assign("x.2", cir::Expr::arg(cir::Arg::int(22))),
                cir::Tail::seq(
                    cir::Stmt::assign(
                        "y",
                        cir::Expr::add(cir::Arg::var("x.1"), cir::Arg::var("x.2")),
                    ),
                    cir::Tail::ret(cir::Expr::arg(cir::Arg::var("y"))),
                ),
            ),
        );

        let actual = fold_root_expr(*expr);
        assert_eq!(actual, expected);
    }
}
