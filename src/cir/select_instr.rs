use super::super::pxir;
use super::*;

/// Folds the CIR argument into a PXIR argument.
fn fold_arg(arg: Arg) -> Box<pxir::Arg> {
    match arg {
        Arg::Int(i) => pxir::Arg::int(i),
        Arg::Var(sym) => pxir::Arg::var(&sym.value),
    }
}

mod assign {
    use super::super::super::pxir;
    use super::super::*;
    use super::fold_arg;

    /// Creates PXIR instructions that read and assign the parsed input to the
    /// destination.
    fn read_instrs(dst: Box<pxir::Arg>) -> Vec<pxir::Instr> {
        vec![
            pxir::Instr::callq("read_int"),
            pxir::Instr::movq(pxir::Arg::reg(pxir::Register::Rax), dst),
        ]
    }

    /// Creates PXIR instructions that assign the source argument to the
    /// destination.
    fn arg_move_instrs(src: Box<pxir::Arg>, dst: Box<pxir::Arg>) -> Vec<pxir::Instr> {
        vec![pxir::Instr::movq(src, dst)]
    }

    /// Creates PXIR instructions that negate the given operand and assign the
    /// result to the destination.
    fn neg_instrs(op: Box<pxir::Arg>, dst: Box<pxir::Arg>) -> Vec<pxir::Instr> {
        if let pxir::Arg::Var(arg_sym) = &*op {
            if let pxir::Arg::Var(dst_sym) = &*dst {
                if arg_sym.value == dst_sym.value {
                    return vec![pxir::Instr::negq(dst)];
                }
            }
        }
        vec![pxir::Instr::movq(op, dst.clone()), pxir::Instr::negq(dst)]
    }

    /// Creates PXIR instructions that add the given operands and assign the
    /// result to the destination.
    fn add_instrs(
        op1: Box<pxir::Arg>,
        op2: Box<pxir::Arg>,
        dst: Box<pxir::Arg>,
    ) -> Vec<pxir::Instr> {
        if let pxir::Arg::Var(sym) = &*op1 {
            if let pxir::Arg::Var(dst_sym) = &*dst {
                if sym.value == dst_sym.value {
                    return vec![pxir::Instr::addq(op2, dst)];
                }
            }
        } else if let pxir::Arg::Var(sym) = &*op2 {
            if let pxir::Arg::Var(dst_sym) = &*dst {
                if sym.value == dst_sym.value {
                    return vec![pxir::Instr::addq(op1, dst)];
                }
            }
        }
        vec![
            pxir::Instr::movq(op1, dst.clone()),
            pxir::Instr::addq(op2, dst),
        ]
    }

    /// Creates PXIR instructions that evaluate the given expresion and assign
    /// the result to the destination.
    pub fn expr_instrs(expr: Expr, dst: Box<pxir::Arg>) -> Vec<pxir::Instr> {
        match expr {
            Expr::Read => read_instrs(dst),
            Expr::Arg(arg) => {
                let arg = fold_arg(*arg);
                arg_move_instrs(arg, dst)
            }
            Expr::Neg(op) => {
                let op = fold_arg(*op);
                neg_instrs(op, dst)
            }
            Expr::Add(op1, op2) => {
                let op1 = fold_arg(*op1);
                let op2 = fold_arg(*op2);
                add_instrs(op1, op2, dst)
            }
        }
    }
}

/// Folds the CIR statment into PXIR instructions.
fn fold_stmt(stmt: Stmt) -> Vec<pxir::Instr> {
    match stmt {
        Stmt::Assign(dst_sym, expr) => {
            let dst = pxir::Arg::var(&dst_sym.value);
            assign::expr_instrs(*expr, dst)
        }
    }
}

/// Folds the CIR tail into PXIR instructions that return by jumping to the
/// given conclusion label.
fn fold_tail(tail: Tail, conclusion_label: &str) -> Vec<pxir::Instr> {
    match tail {
        Tail::Seq(stmt, tail) => {
            let mut instrs = fold_stmt(*stmt);
            instrs.extend(fold_tail(*tail, conclusion_label));
            instrs
        }
        Tail::Ret(expr) => {
            let mut instrs = assign::expr_instrs(*expr, pxir::Arg::reg(pxir::Register::Rax));
            instrs.push(pxir::Instr::jumpq(conclusion_label));
            instrs
        }
    }
}

/// Folds the CIR program into a PXIR program.
pub fn fold_program(program: Program) -> pxir::Program {
    let mut blocks = HashMap::new();
    for (label, tail) in program.tails {
        let label = pxir::Label { value: label.value };
        // FIXME: Different blocks may need different conclusion labels.
        let instrs = fold_tail(tail, "conclusion");
        let block = pxir::Block::new(instrs);
        blocks.insert(label, block);
    }
    pxir::Program {
        info: pxir::ProgramInfo {},
        blocks,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::pxir;
    use super::super::*;
    use super::fold_tail;

    #[test]
    fn read() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::read()),
            Tail::ret(Expr::arg(Arg::var("x"))),
        );
        let expected = vec![
            pxir::Instr::callq("read_int"),
            pxir::Instr::movq(pxir::Arg::reg(pxir::Register::Rax), pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("read_conclusion"),
        ];
        let actual = fold_tail(*tail, "read_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn basic_add_and_neg() {
        let tail = Tail::seq(
            Stmt::assign("v200000", Expr::neg(Arg::int(10))),
            Tail::ret(Expr::add(Arg::int(52), Arg::var("v200000"))),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(10), pxir::Arg::var("v200000")),
            pxir::Instr::negq(pxir::Arg::var("v200000")),
            pxir::Instr::movq(pxir::Arg::int(52), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::addq(
                pxir::Arg::var("v200000"),
                pxir::Arg::reg(pxir::Register::Rax),
            ),
            pxir::Instr::jumpq("basic_add_and_neg_conclusion"),
        ];
        let actual = fold_tail(*tail, "basic_add_and_neg_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn add() {
        let tail = Tail::seq(
            Stmt::assign("x.1", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x.2", Expr::arg(Arg::int(22))),
                Tail::seq(
                    Stmt::assign("y", Expr::add(Arg::var("x.1"), Arg::var("x.2"))),
                    Tail::ret(Expr::arg(Arg::var("y"))),
                ),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x.1")),
            pxir::Instr::movq(pxir::Arg::int(22), pxir::Arg::var("x.2")),
            pxir::Instr::movq(pxir::Arg::var("x.1"), pxir::Arg::var("y")),
            pxir::Instr::addq(pxir::Arg::var("x.2"), pxir::Arg::var("y")),
            pxir::Instr::movq(pxir::Arg::var("y"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("add_conclusion"),
        ];
        let actual = fold_tail(*tail, "add_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_in_place_left_op() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x", Expr::add(Arg::var("x"), Arg::int(22))),
                Tail::ret(Expr::arg(Arg::var("x"))),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x")),
            pxir::Instr::addq(pxir::Arg::int(22), pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("add_in_place_left_op_conclusion"),
        ];
        let actual = fold_tail(*tail, "add_in_place_left_op_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_in_place_right_op() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x", Expr::add(Arg::int(22), Arg::var("x"))),
                Tail::ret(Expr::arg(Arg::var("x"))),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x")),
            pxir::Instr::addq(pxir::Arg::int(22), pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("add_in_place_right_op_conclusion"),
        ];
        let actual = fold_tail(*tail, "add_in_place_right_op_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_in_place_both_ops() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x", Expr::add(Arg::var("x"), Arg::var("x"))),
                Tail::ret(Expr::arg(Arg::var("x"))),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x")),
            pxir::Instr::addq(pxir::Arg::var("x"), pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("add_in_place_both_ops_conclusion"),
        ];
        let actual = fold_tail(*tail, "add_in_place_both_ops_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn neg() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("y", Expr::neg(Arg::var("x"))),
                Tail::ret(Expr::arg(Arg::var("y"))),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::var("y")),
            pxir::Instr::negq(pxir::Arg::var("y")),
            pxir::Instr::movq(pxir::Arg::var("y"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("neg_conclusion"),
        ];
        let actual = fold_tail(*tail, "neg_conclusion");
        assert_eq!(actual, expected);
    }

    #[test]
    fn neg_in_place() {
        let tail = Tail::seq(
            Stmt::assign("x", Expr::arg(Arg::int(20))),
            Tail::seq(
                Stmt::assign("x", Expr::neg(Arg::var("x"))),
                Tail::ret(Expr::arg(Arg::var("x"))),
            ),
        );
        let expected = vec![
            pxir::Instr::movq(pxir::Arg::int(20), pxir::Arg::var("x")),
            pxir::Instr::negq(pxir::Arg::var("x")),
            pxir::Instr::movq(pxir::Arg::var("x"), pxir::Arg::reg(pxir::Register::Rax)),
            pxir::Instr::jumpq("neg_in_place_conclusion"),
        ];
        let actual = fold_tail(*tail, "neg_in_place_conclusion");
        assert_eq!(actual, expected);
    }
}
