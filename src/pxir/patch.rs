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
        _ => vec![instr],
    }
}

pub fn fold_block(block: Block) -> Block {
    Block {
        info: block.info,
        instrs: block.instrs.into_iter().map(fold_instr).flatten().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::fold_block;

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
            info: BlockInfo {},
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
