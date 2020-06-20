use super::*;

pub struct Ctx {
    /// Space needed for stack variables in bytes.
    pub stack_space: i64,

    /// Maps symbols to its storage location in the stack frame. Storage
    /// location is represented as an offset in bytes from the base pointer.
    pub sym_to_home: HashMap<Symbol, i64>,
}

impl Ctx {
    pub fn new() -> Ctx {
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

    pub fn fold_block(&mut self, block: Block) -> Block {
        Block {
            info: block.info,
            instrs: block
                .instrs
                .into_iter()
                .map(|i| self.fold_instr(i))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::Ctx;

    #[test]
    fn fold_block() {
        let instrs = vec![
            Instr::movq(Arg::int(10), Arg::var("x.1")),
            Instr::negq(Arg::var("x.1")),
            Instr::movq(Arg::var("x.1"), Arg::var("x.2")),
            Instr::movq(Arg::int(52), Arg::var("x.2")),
            Instr::movq(Arg::var("x.2"), Arg::reg(Register::Rax)),
        ];
        let block = Block {
            info: BlockInfo {},
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
        let mut ctx = Ctx::new();
        let actual = ctx.fold_block(block);
        assert_eq!(actual.instrs, expected_instrs);
        assert_eq!(ctx.stack_space, 16);
    }
}
