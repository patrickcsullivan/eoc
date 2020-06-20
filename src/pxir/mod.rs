// ! PXIR (Pseudo-x86 Intermediate Representation)

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Arg {
    Int(i64),
    Reg(Register),
    Deref(Register, i64),
    Var(Box<Symbol>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    Addq(Box<Arg>, Box<Arg>),
    Subq(Box<Arg>, Box<Arg>),
    Movq(Box<Arg>, Box<Arg>),
    Retq,
    Negq(Box<Arg>),
    Callq(Box<Label>),
    Pushq(Box<Arg>),
    Popq(Box<Arg>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockInfo {}

#[derive(Clone, Debug)]
pub struct Block {
    info: BlockInfo,
    instrs: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramInfo {}

#[derive(Clone, Debug)]
pub struct Program {
    info: ProgramInfo,
    blocks: HashMap<Label, Block>,
}
