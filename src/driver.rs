use super::cir;
use super::pxir;
use super::rir;
use super::rir::ExprFolder;

pub fn drive(expr: rir::Expr) -> String {
    // RIR folds
    let mut uniquify_ctx = rir::uniquify::ExprUniquifier::new(12345);
    let expr = uniquify_ctx.fold(Box::new(expr));
    let mut arg_simplify_ctx = rir::arg_simplify::ExprArgSimplifier::new(uniquify_ctx.counter);
    let expr = arg_simplify_ctx.fold(expr);
    let prog = rir::Program::new(expr);

    // CIR folds
    let prog = rir::explicate::fold_program(prog);
    let prog = cir::uncover::fold_program(prog);

    // PXIR folds
    let prog = cir::select_instr::fold_program(prog);
    // let expected = vec![
    //     pxir::Instr::movq(pxir::Arg::int(10), pxir::Arg::var("v12345")),
    //     pxir::Instr::negq(pxir::Arg::var("v12345")),
    //     pxir::Instr::movq(pxir::Arg::int(52), pxir::Arg::reg(pxir::Register::Rax)),
    //     pxir::Instr::addq(
    //         pxir::Arg::var("v12345"),
    //         pxir::Arg::reg(pxir::Register::Rax),
    //     ),
    //     pxir::Instr::jumpq("conclusion"),
    // ];
    // let label = pxir::Label {
    //     value: "start".to_string(),
    // };
    // assert_eq!(prog.blocks.get(&label).unwrap().instrs, expected);

    let prog = pxir::assign_homes::fold_program(prog);
    let prog = pxir::patch::fold_program(prog);

    // Prepare to write.
    let start_label = pxir::Label {
        value: "start".to_string(),
    };
    let start_block = prog.blocks.get(&start_label).unwrap();
    let start_stack_space = adjusted_stack_space(start_block.info.stack_space);
    let main_label = pxir::Label {
        value: "main".to_string(),
    };
    let main_block = build_main_block(start_stack_space, &start_label);
    let conclusion_label = pxir::Label {
        value: "conclusion".to_string(),
    };
    let conclusion_block = build_conclusion_block(start_stack_space);

    // Write x86
    use pxir::write_block;
    use std::fmt::Write;
    let mut out = "".to_string();
    write_block(&mut out, &start_label, start_block).unwrap();
    writeln!(&mut out).unwrap();
    writeln!(&mut out, "\t.globl main").unwrap();
    write_block(&mut out, &main_label, &main_block).unwrap();
    write_block(&mut out, &conclusion_label, &conclusion_block).unwrap();

    out
}

fn adjusted_stack_space(stack_size: i64) -> i64 {
    if stack_size % 16 == 0 {
        stack_size
    } else {
        stack_size + 8
    }
}

fn build_main_block(stack_size: i64, jump_to: &pxir::Label) -> pxir::Block {
    let instrs = vec![
        pxir::Instr::pushq(pxir::Arg::reg(pxir::Register::Rbp)),
        pxir::Instr::movq(
            pxir::Arg::reg(pxir::Register::Rsp),
            pxir::Arg::reg(pxir::Register::Rbp),
        ),
        pxir::Instr::subq(
            pxir::Arg::int(stack_size),
            pxir::Arg::reg(pxir::Register::Rsp),
        ),
        pxir::Instr::jumpq(&jump_to.value),
    ];
    pxir::Block::new(instrs)
}

fn build_conclusion_block(stack_size: i64) -> pxir::Block {
    let instrs = vec![
        pxir::Instr::addq(
            pxir::Arg::int(stack_size),
            pxir::Arg::reg(pxir::Register::Rsp),
        ),
        pxir::Instr::popq(pxir::Arg::reg(pxir::Register::Rbp)),
        pxir::Instr::retq(),
    ];
    pxir::Block::new(instrs)
}
