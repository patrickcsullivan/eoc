// ! PXIR (Pseudo-x86 Intermediate Representation)

pub mod assign_homes;
pub mod patch;
mod write;

pub use write::write_block;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: String,
}

impl Symbol {
    pub fn new(value: &str) -> Symbol {
        Symbol {
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arg {
    Int(i64),
    Reg(Register),
    Deref(Register, i64),
    Var(Box<Symbol>),
}

impl Arg {
    pub fn int(i: i64) -> Box<Arg> {
        Box::new(Arg::Int(i))
    }

    pub fn reg(reg: Register) -> Box<Arg> {
        Box::new(Arg::Reg(reg))
    }

    pub fn deref(reg: Register, offset: i64) -> Box<Arg> {
        Box::new(Arg::Deref(reg, offset))
    }

    pub fn var(s: &str) -> Box<Arg> {
        Box::new(Arg::Var(Box::new(Symbol::new(s))))
    }

    pub fn is_dref(&self) -> bool {
        match self {
            Arg::Deref(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label {
    pub value: String,
}

impl Label {
    pub fn new(value: &str) -> Box<Label> {
        Box::new(Label {
            value: value.to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    Addq { src: Box<Arg>, dst: Box<Arg> },
    Subq { src: Box<Arg>, dst: Box<Arg> },
    Movq { src: Box<Arg>, dst: Box<Arg> },
    Negq(Box<Arg>),
    Pushq(Box<Arg>),
    Popq(Box<Arg>),
    Callq(Box<Label>),
    Jumpq(Box<Label>),
    Retq,
}

impl Instr {
    pub fn addq(src: Box<Arg>, dst: Box<Arg>) -> Instr {
        Instr::Addq { src, dst }
    }

    pub fn subq(src: Box<Arg>, dst: Box<Arg>) -> Instr {
        Instr::Subq { src, dst }
    }

    pub fn movq(src: Box<Arg>, dst: Box<Arg>) -> Instr {
        Instr::Movq { src, dst }
    }

    pub fn retq() -> Instr {
        Instr::Retq
    }

    pub fn negq(dst: Box<Arg>) -> Instr {
        Instr::Negq(dst)
    }

    pub fn callq(label: &str) -> Instr {
        Instr::Callq(Label::new(label))
    }

    pub fn jumpq(label: &str) -> Instr {
        Instr::Jumpq(Label::new(label))
    }

    pub fn pushq(dst: Box<Arg>) -> Instr {
        Instr::Pushq(dst)
    }

    pub fn popq(src: Box<Arg>) -> Instr {
        Instr::Popq(src)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockInfo {
    /// Space needed for stack variables in bytes.
    pub stack_space: i64,
}

impl BlockInfo {
    pub fn new() -> BlockInfo {
        BlockInfo { stack_space: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub info: BlockInfo,
    pub instrs: Vec<Instr>,
}

impl Block {
    pub fn new(instrs: Vec<Instr>) -> Block {
        Block {
            info: BlockInfo::new(),
            instrs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramInfo {}

#[derive(Clone, Debug)]
pub struct Program {
    pub info: ProgramInfo,
    pub blocks: HashMap<Label, Block>,
}
