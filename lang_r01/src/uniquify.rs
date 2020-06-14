use super::ast::{Expr, ExprFolder, Program, ProgramFolder, Symbol};
use std::collections::HashMap;

/// Maintains state necessary for uniquify-ing the variable names in an AST.
struct ExprUniquifier {
    counter: u64,

    /// Maps variable names from source code to generated, uniqued variable
    /// names.
    sym_table: HashMap<Box<Symbol>, Box<Symbol>>,
}

impl ExprUniquifier {
    pub fn new() -> ExprUniquifier {
        ExprUniquifier {
            counter: 12345,
            sym_table: HashMap::new(),
        }
    }

    pub fn new_sym(&mut self) -> Box<Symbol> {
        let sym = Box::new(Symbol::new(format!("v{}", self.counter)));
        self.counter += 1;
        sym
    }
}

impl ExprFolder for ExprUniquifier {
    fn fold_var(&mut self, s: Box<Symbol>) -> Box<Expr> {
        let gen = self.sym_table.get(&s).expect("undefined variable");
        Box::new(Expr::Var(gen.clone()))
    }

    fn fold_let(&mut self, sym: Box<Symbol>, e: Box<Expr>, body: Box<Expr>) -> Box<Expr> {
        let gen = self.new_sym();
        self.sym_table.insert(sym, gen.clone());
        Box::new(Expr::Let(gen, self.fold(e), self.fold(body)))
    }
}

pub struct ProgramUniquifier {}

impl ProgramFolder for ProgramUniquifier {
    fn fold(&mut self, p: Program) -> Program {
        let mut ctx = ExprUniquifier::new();
        Program {
            expr: ctx.fold(p.expr),
        }
    }
}
//
// pub struct Ctx {
//     counter: u64,
//     sym_table: HashMap<Symbol, Symbol>,
// }

// impl Ctx {
//     const COUNTER_START: u64 = 12345;

//     /// Returns a new Ctx.
//     pub fn new() -> Ctx {
//         Ctx {
//             counter: Ctx::COUNTER_START,
//             sym_table: HashMap::new(),
//         }
//     }

//     /// Returns `true` if the symbol table contains the given source symbol.
//     pub fn contains_src(&self, sym: &Symbol) -> bool {
//         self.sym_table.contains_key(sym)
//     }

//     /// Creates a generated symbol for the source symbol and inserts a clone of
//     /// the symbol into the symbol table
//     pub fn insert_sym(&mut self, sym: &Symbol) -> &Symbol {
//         if self.sym_table.get(sym).is_none() {
//             self.counter += 1;
//             let gen = Symbol {
//                 Value: format!("v{}", self.counter),
//             };
//             self.sym_table.insert(sym.clone(), gen);
//         }

//         self.get_gen(&sym).unwrap()
//     }

//     /// Returns the generated symbol that is associated with the given source
//     /// symbol.
//     pub fn get_gen(&self, sym: &Symbol) -> Option<&Symbol> {
//         self.sym_table.get(sym)
//     }
// }

// pub fn uniquify(p: Program) {
//     let mut ctx = Ctx::new();
//     uniquify_exper(&mut ctx, p.Expr);
// }

// fn uniquify_exper(ctx: &mut Ctx, expr: Box<Expr>) -> Box<Expr> {
//     match *expr {
//         Expr::Var(sym) => {
//             let gen = ctx.get_gen(&sym).expect("undeclared variable");
//             Box::new(Expr::Var(gen.clone()))
//         }
//         Expr::Let(sym, expr, body) => {
//             let gen = ctx.insert_sym(&sym);
//             let expr = uniquify_exper(ctx, expr);
//             let body = uniquify_exper(ctx, body);
//             Box::new(Expr::Let(gen.clone(), expr, body))
//         }
//         _ => expr,
//     }
// }
