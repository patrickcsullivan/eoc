use super::*;

fn fold_instr(instr: Instr) -> Vec<Instr> {
    match instr {
        Instr::Movq { src, dst } => {
            if src.is_dref() && dst.is_dref() {
                return vec![
                    Instr::movq(src, Arg::reg(Register::Rax)),
                    Instr::movq(Arg::reg(Register::Rax), dst),
                ];
            }
            vec![Instr::movq(src, dst)]
        }
        Instr::Addq { src, dst } => {
            if src.is_dref() && dst.is_dref() {
                return vec![
                    Instr::movq(src, Arg::reg(Register::Rax)),
                    Instr::addq(Arg::reg(Register::Rax), dst),
                ];
            }
            vec![Instr::addq(src, dst)]
        }
        Instr::Subq { src, dst } => {
            if src.is_dref() && dst.is_dref() {
                return vec![
                    Instr::movq(src, Arg::reg(Register::Rax)),
                    Instr::subq(Arg::reg(Register::Rax), dst),
                ];
            }
            vec![Instr::subq(src, dst)]
        }
        _ => vec![instr],
    }
}

fn fold_block(block: Block) -> Block {
    Block {
        info: block.info,
        instrs: block.instrs.into_iter().map(fold_instr).flatten().collect(),
    }
}

pub fn fold_program(program: Program) -> Program {
    let mut blocks = HashMap::new();
    for (label, block) in program.blocks {
        let block = fold_block(block);
        blocks.insert(label, block);
    }
    Program {
        info: program.info,
        blocks,
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::fold_block;

    #[test]
    fn basic_add_and_neg() {
        let instrs = vec![
            Instr::movq(Arg::int(10), Arg::deref(Register::Rbp, -8)),
            Instr::negq(Arg::deref(Register::Rbp, -8)),
            Instr::movq(Arg::int(52), Arg::reg(Register::Rax)),
            Instr::addq(Arg::deref(Register::Rbp, -8), Arg::reg(Register::Rax)),
            Instr::jumpq("basic_add_and_neg_conclusion"),
        ];
        let block = Block {
            info: BlockInfo { stack_space: 8 },
            instrs: instrs.clone(),
        };
        let expected_instrs = instrs;
        let actual = fold_block(block);
        assert_eq!(actual.instrs, expected_instrs);
        assert_eq!(actual.info.stack_space, 8);
    }

    #[test]
    fn multiple_mem_args() {
        let instrs = vec![
            Instr::movq(Arg::int(42), Arg::deref(Register::Rbp, -8)),
            Instr::movq(
                Arg::deref(Register::Rbp, -8),
                Arg::deref(Register::Rbp, -16),
            ),
            Instr::movq(Arg::deref(Register::Rbp, -16), Arg::reg(Register::Rax)),
        ];
        let block = Block {
            info: BlockInfo { stack_space: 16 },
            instrs,
        };
        let expected_instrs = vec![
            Instr::movq(Arg::int(42), Arg::deref(Register::Rbp, -8)),
            Instr::movq(Arg::deref(Register::Rbp, -8), Arg::reg(Register::Rax)),
            Instr::movq(Arg::reg(Register::Rax), Arg::deref(Register::Rbp, -16)),
            Instr::movq(Arg::deref(Register::Rbp, -16), Arg::reg(Register::Rax)),
        ];
        let actual = fold_block(block);
        assert_eq!(actual.instrs, expected_instrs);
    }
}
