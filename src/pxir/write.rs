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
            Instr::Addq { src, dst } => write!(f, "addq {} {}", *src, *dst),
            Instr::Subq { src, dst } => write!(f, "subq {} {}", *src, *dst),
            Instr::Movq { src, dst } => write!(f, "movq {} {}", *src, *dst),
            Instr::Negq(dst) => write!(f, "negq {}", *dst),
            Instr::Pushq(src) => write!(f, "pushq {}", *src),
            Instr::Popq(dst) => write!(f, "popq {}", *dst),
            Instr::Callq(_) => panic!("unimplemented"),
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
            Rsp => "rsp",
            Rbp => "rbp",
            Rax => "rax",
            Rbx => "rbx",
            Rcx => "rcx",
            Rdx => "rdx",
            Rsi => "rsi",
            Rdi => "rdi",
            R8 => "r8",
            R9 => "r9",
            R10 => "r10",
            R11 => "r11",
            R12 => "r12",
            R13 => "r13",
            R14 => "r14",
            R15 => "r15",
        };
        write!(f, "%{}", disp)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
