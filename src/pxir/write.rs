use super::*;
use std::fmt;
use std::fmt::Write;

pub fn write_block(s: &mut String, label: &Label, block: &Block) -> fmt::Result {
    writeln!(s, "{}:", label)?;
    for instr in &block.instrs {
        writeln!(s, "\t{}", instr)?;
    }
    Ok(())
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::Addq { src, dst } => write!(f, "addq {}, {}", *src, *dst),
            Instr::Subq { src, dst } => write!(f, "subq {}, {}", *src, *dst),
            Instr::Movq { src, dst } => write!(f, "movq {}, {}", *src, *dst),
            Instr::Negq(dst) => write!(f, "negq {}", *dst),
            Instr::Pushq(src) => write!(f, "pushq {}", *src),
            Instr::Popq(dst) => write!(f, "popq {}", *dst),
            Instr::Callq(label) => write!(f, "callq {}", *label),
            Instr::Jumpq(label) => write!(f, "jmp {}", *label),
            Instr::Retq => write!(f, "retq"),
        }
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Int(i) => write!(f, "${}", i),
            Arg::Reg(r) => write!(f, "{}", r),
            Arg::Deref(r, off) => write!(f, "{}({})", off, r),
            Arg::Var(sym) => write!(f, "var<{}>", sym.value),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = match self {
            Register::Rsp => "rsp",
            Register::Rbp => "rbp",
            Register::Rax => "rax",
            Register::Rbx => "rbx",
            Register::Rcx => "rcx",
            Register::Rdx => "rdx",
            Register::Rsi => "rsi",
            Register::Rdi => "rdi",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        };
        write!(f, "%{}", disp)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
