use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Register {
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

#[derive(Clone, Copy, Debug)]
enum Arg {
    Int(i64),
    Reg(Register),
    Deref(Register, i64),
}

type Label<'a> = &'a str;

#[derive(Clone, Copy, Debug)]
enum Instr<'a> {
    Addq(Arg, Arg),
    Subq(Arg, Arg),
    Movq(Arg, Arg),
    Retq,
    Negq(Arg),
    Callq(Label<'a>),
    Pushq(Arg),
    Popq(Arg),
}

type Info = ();

#[derive(Clone, Debug)]
struct Block<'a> {
    Info: Info,
    Instrs: Vec<Instr<'a>>,
}

#[derive(Clone, Debug)]
struct Program<'a> {
    Info: Info,
    Blocks: HashMap<Label<'a>, Block<'a>>,
}
