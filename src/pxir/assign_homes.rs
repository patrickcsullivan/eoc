use super::*;

struct Ctx {
    /// Space needed for stack variables in bytes.
    stack_space: i64,

    /// Maps symbols to its storage location in the stack frame. Storage
    /// location is represented as an offset in bytes from the base pointer.
    sym_to_home: HashMap<Symbol, i64>,
}

impl Ctx {
    fn new() -> Ctx {
        Ctx {
            stack_space: 0,
            sym_to_home: HashMap::new(),
        }
    }

    fn get_home(&mut self, sym: &Symbol) -> Box<Arg> {
        if let Some(offset) = self.sym_to_home.get(sym) {
            return Arg::deref(Register::Rbp, *offset);
        }
        self.stack_space += 8;
        let offset = self.stack_space * -1;
        self.sym_to_home.insert(sym.clone(), offset);
        Arg::deref(Register::Rbp, offset)
    }

    fn fold_arg(&mut self, arg: Box<Arg>) -> Box<Arg> {
        match *arg {
            Arg::Var(sym) => self.get_home(&sym),
            _ => arg,
        }
    }

    fn fold_instr(&mut self, instr: Instr) -> Instr {
        match instr {
            Instr::Addq { src, dst } => Instr::addq(self.fold_arg(src), self.fold_arg(dst)),
            Instr::Subq { src, dst } => Instr::subq(self.fold_arg(src), self.fold_arg(dst)),
            Instr::Movq { src, dst } => Instr::movq(self.fold_arg(src), self.fold_arg(dst)),
            Instr::Negq(dst) => Instr::negq(self.fold_arg(dst)),
            _ => instr,
        }
    }
}

fn fold_block(block: Block) -> Block {
    let mut ctx = Ctx::new();
    let instrs = block
        .instrs
        .into_iter()
        .map(|i| ctx.fold_instr(i))
        .collect();

    Block {
        info: BlockInfo {
            stack_space: ctx.stack_space,
        },
        instrs,
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
            Instr::movq(Arg::int(10), Arg::var("v200000")),
            Instr::negq(Arg::var("v200000")),
            Instr::movq(Arg::int(52), Arg::reg(Register::Rax)),
            Instr::addq(Arg::var("v200000"), Arg::reg(Register::Rax)),
            Instr::jumpq("basic_add_and_neg_conclusion"),
        ];
        let block = Block {
            info: BlockInfo::new(),
            instrs,
        };
        let expected_instrs = vec![
            Instr::movq(Arg::int(10), Arg::deref(Register::Rbp, -8)),
            Instr::negq(Arg::deref(Register::Rbp, -8)),
            Instr::movq(Arg::int(52), Arg::reg(Register::Rax)),
            Instr::addq(Arg::deref(Register::Rbp, -8), Arg::reg(Register::Rax)),
            Instr::jumpq("basic_add_and_neg_conclusion"),
        ];
        let actual = fold_block(block);
        assert_eq!(actual.instrs, expected_instrs);
        assert_eq!(actual.info.stack_space, 8);
    }

    #[test]
    fn moves_and_neg() {
        let instrs = vec![
            Instr::movq(Arg::int(10), Arg::var("x.1")),
            Instr::negq(Arg::var("x.1")),
            Instr::movq(Arg::var("x.1"), Arg::var("x.2")),
            Instr::movq(Arg::int(52), Arg::var("x.2")),
            Instr::movq(Arg::var("x.2"), Arg::reg(Register::Rax)),
        ];
        let block = Block {
            info: BlockInfo::new(),
            instrs,
        };
        let expected_instrs = vec![
            Instr::movq(Arg::int(10), Arg::deref(Register::Rbp, -8)),
            Instr::negq(Arg::deref(Register::Rbp, -8)),
            Instr::movq(
                Arg::deref(Register::Rbp, -8),
                Arg::deref(Register::Rbp, -16),
            ),
            Instr::movq(Arg::int(52), Arg::deref(Register::Rbp, -16)),
            Instr::movq(Arg::deref(Register::Rbp, -16), Arg::reg(Register::Rax)),
        ];
        let actual = fold_block(block);
        assert_eq!(actual.instrs, expected_instrs);
        assert_eq!(actual.info.stack_space, 16);
    }
}
