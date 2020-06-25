use super::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn get_arg_var(arg: Arg) -> Option<Symbol> {
    match arg {
        Arg::Var(sym) => Some(*sym),
        _ => None,
    }
}

fn vars_read(instr: &Instr) -> HashSet<Symbol> {
    let args_read = match instr.clone() {
        Instr::Addq { src, dst } => vec![src, dst],
        Instr::Subq { src, dst } => vec![src, dst],
        Instr::Movq { src, .. } => vec![src],
        Instr::Negq(dst) => vec![dst],
        Instr::Pushq(src) => vec![src],
        Instr::Popq(_) => vec![],
        Instr::Callq(_) => vec![],
        Instr::Jumpq(_) => vec![],
        Instr::Retq => vec![],
    };
    HashSet::from_iter(args_read.into_iter().filter_map(|a| get_arg_var(*a)))
}

fn vars_written(instr: &Instr) -> HashSet<Symbol> {
    let args_written = match instr.clone() {
        Instr::Addq { dst, .. } => vec![dst],
        Instr::Subq { dst, .. } => vec![dst],
        Instr::Movq { dst, .. } => vec![dst],
        Instr::Negq(dst) => vec![dst],
        Instr::Pushq(_) => vec![],
        Instr::Popq(dst) => vec![dst],
        Instr::Callq(_) => vec![],
        Instr::Jumpq(_) => vec![],
        Instr::Retq => vec![],
    };
    HashSet::from_iter(args_written.into_iter().filter_map(|a| get_arg_var(*a)))
}

fn live_before_instr(instr: &Instr, live_after: &HashSet<Symbol>) -> HashSet<Symbol> {
    let written = vars_written(instr);
    let read = vars_read(instr);
    let mut live_before = live_after
        .difference(&written)
        .cloned()
        .collect::<Vec<Symbol>>();
    live_before.append(&mut read.into_iter().collect::<Vec<Symbol>>());
    HashSet::from_iter(live_before.into_iter())
}

/// Gets the set of variables that are live after an instruction for each
/// instruction in the block.
pub fn uncover_live(block: &Block) -> Vec<HashSet<Symbol>> {
    // We build the list of live after sets in reverse order.
    let mut live_after_sets = vec![];
    live_after_sets.push(HashSet::new());
    for instr in block.instrs.iter().rev() {
        // There will always be at least one set so unwrapping is ok.
        let prev_live_after = live_after_sets.last().unwrap();
        let live_before = live_before_instr(instr, prev_live_after);
        live_after_sets.push(live_before);
    }
    live_after_sets.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::uncover_live;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn simple() {
        let instrs = vec![
            Instr::movq(Arg::int(1), Arg::var("v")),
            Instr::movq(Arg::int(46), Arg::var("w")),
            Instr::movq(Arg::var("v"), Arg::var("x")),
            Instr::addq(Arg::int(7), Arg::var("x")),
            Instr::movq(Arg::var("x"), Arg::var("y")),
            Instr::addq(Arg::int(4), Arg::var("y")),
            Instr::movq(Arg::var("x"), Arg::var("z")),
            Instr::addq(Arg::var("w"), Arg::var("z")),
            Instr::movq(Arg::var("y"), Arg::var("t.1")),
            Instr::negq(Arg::var("t.1")),
            Instr::movq(Arg::var("z"), Arg::reg(Register::Rax)),
            Instr::addq(Arg::var("t.1"), Arg::reg(Register::Rax)),
            Instr::jumpq("conclusion"),
        ];
        let block = Block {
            info: BlockInfo { stack_space: 48 },
            instrs,
        };
        let expected = vec![
            symbol_set(vec![]),
            symbol_set(vec!["v"]),
            symbol_set(vec!["v", "w"]),
            symbol_set(vec!["w", "x"]),
            symbol_set(vec!["w", "x"]),
            symbol_set(vec!["w", "x", "y"]),
            symbol_set(vec!["w", "x", "y"]),
            symbol_set(vec!["w", "y", "z"]),
            symbol_set(vec!["y", "z"]),
            symbol_set(vec!["z", "t.1"]),
            symbol_set(vec!["z", "t.1"]),
            symbol_set(vec!["t.1"]),
            symbol_set(vec![]),
            symbol_set(vec![]),
        ];
        let actual = uncover_live(&block);
        assert_eq!(actual, expected);
    }

    fn symbol_set(names: Vec<&str>) -> HashSet<Symbol> {
        HashSet::from_iter(names.iter().map(|s| Symbol::new(s)))
    }
}
