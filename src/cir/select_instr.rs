use super::super::pxir;
use super::*;

pub fn fold_arg(arg: Box<Arg>) -> Box<pxir::Arg> {
    match *arg {
        Arg::Int(i) => pxir::Arg::int(i),
        Arg::Var(sym) => pxir::Arg::var(&sym.value),
    }
}

pub fn fold_read_expr(dst_sym: Box<Symbol>) -> Vec<pxir::Instr> {
    vec![
        pxir::Instr::callq("read_int"),
        pxir::Instr::movq(
            pxir::Arg::reg(pxir::Register::Rax),
            pxir::Arg::var(&dst_sym.value),
        ),
    ]
}

pub fn fold_arg_expr(dst_sym: Box<Symbol>, arg: Box<Arg>) -> pxir::Instr {
    pxir::Instr::movq(fold_arg(arg), pxir::Arg::var(&dst_sym.value))
}

pub fn fold_neg_expr(dst_sym: Box<Symbol>, arg: Box<Arg>) -> Vec<pxir::Instr> {
    let dst = pxir::Arg::var(&dst_sym.value);
    let arg = fold_arg(arg);

    if let pxir::Arg::Var(sym) = &*arg {
        if sym.value == dst_sym.value {
            return vec![pxir::Instr::negq(dst)];
        }
    }
    vec![pxir::Instr::movq(arg, dst.clone()), pxir::Instr::negq(dst)]
}

pub fn fold_add_expr(dst_sym: Box<Symbol>, arg1: Box<Arg>, arg2: Box<Arg>) -> Vec<pxir::Instr> {
    let dst = pxir::Arg::var(&dst_sym.value);
    let arg1 = fold_arg(arg1);
    let arg2 = fold_arg(arg2);

    if let pxir::Arg::Var(sym) = &*arg1 {
        if sym.value == dst_sym.value {
            return vec![pxir::Instr::addq(arg2, dst)];
        }
    } else if let pxir::Arg::Var(sym) = &*arg2 {
        if sym.value == dst_sym.value {
            return vec![pxir::Instr::addq(arg1, dst)];
        }
    }
    vec![
        pxir::Instr::movq(arg1, dst.clone()),
        pxir::Instr::addq(arg2, dst),
    ]
}

pub fn fold_assign(dst_sym: Box<Symbol>, expr: Box<Expr>) -> Vec<pxir::Instr> {
    match *expr {
        Expr::Read => fold_read_expr(dst_sym),
        Expr::Arg(arg) => vec![fold_arg_expr(dst_sym, arg)],
        Expr::Neg(arg) => fold_neg_expr(dst_sym, arg),
        Expr::Add(arg1, arg2) => fold_add_expr(dst_sym, arg1, arg2),
    }
}

pub fn fold_stmt(stmt: Stmt) -> Vec<pxir::Instr> {
    match stmt {
        Stmt::Assign(dst_sym, expr) => fold_assign(dst_sym, expr),
    }
}

pub fn fold_tail(tail: Tail, conclusion_label: &str) -> Vec<pxir::Instr> {
    match tail {
        Tail::Seq(stmt, tail) => {
            let mut instrs = fold_stmt(*stmt);
            instrs.extend(fold_tail(*tail, conclusion_label));
            instrs
        }
        Tail::Ret(expr) => fold_assign(dst_sym: Box<Symbol>, expr: Box<Expr>),
    }
}
// pub fn fold_program(p: Program) -> Program {
//     let mut ctx = Ctx::new();
//     for t in p.tails.values() {
//         ctx.fold_tail(t);
//     }

//     Program {
//         info: Info {
//             symbols: ctx.symbols,
//         },
//         tails: p.tails,
//     }
// }

#[cfg(test)]
mod tests {
    use super::super::*;
    // use super::fold_program;
}
