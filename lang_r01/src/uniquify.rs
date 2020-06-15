use super::ast::{Expr, ExprFolder, Program, ProgramFolder, Symbol};
use std::collections::HashMap;

/// Maintains state necessary for uniquify-ing the variable names in an AST.
pub struct ExprUniquifier {
    pub counter: u64,

    /// Maps variable names from source code to generated uniqued variable
    /// names. Contains only variables that are currently in scope.
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
        // Fold the value expression first.
        let folded_val = self.fold(e);

        // If the new let symbol shadows an existing variable then hold on to
        // the existing unique symbol for that variable.
        let old_unq_sym = self.sym_table.remove(&sym);

        // Create a new unique symbol for the symbol in the let.
        let gen = self.new_sym();
        self.sym_table.insert(sym.clone(), gen.clone());

        // Fold the body expression with the new unique symbol in the symbol
        // table.
        let folded_body = self.fold(body);

        if let Some(old_unq_sym) = old_unq_sym {
            // Put the unique symbol for the shadowed variable back in the
            // symbol table.
            self.sym_table.insert(sym, old_unq_sym);
        } else {
            // Remove the symbol from the symbol table since it will be out of
            // scope for other parts of the AST.
            self.sym_table.remove(&sym);
        }

        Box::new(Expr::Let(gen, folded_val, folded_body))
    }
}

pub struct ProgramUniquifier {}

impl ProgramFolder for ProgramUniquifier {
    fn fold(&mut self, p: Program) -> Program {
        let mut ctx = ExprUniquifier::new();
        Program::new(ctx.fold(p.expr))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Program, ProgramFolder};
    use crate::uniquify;

    #[test]
    fn uniquify() {
        let prog = Program::new(Expr::let_bind(
            "my_var",
            Expr::int(42),
            Expr::let_bind(
                "input",
                Expr::read(),
                Expr::let_bind(
                    "my_var",
                    Expr::add(Expr::var("my_var"), Expr::neg(Expr::var("input"))),
                    Expr::var("my_var"),
                ),
            ),
        ));

        let expected = Program::new(Expr::let_bind(
            "v12345",
            Expr::int(42),
            Expr::let_bind(
                "v12346",
                Expr::read(),
                Expr::let_bind(
                    "v12347",
                    Expr::add(Expr::var("v12345"), Expr::neg(Expr::var("v12346"))),
                    Expr::var("v12347"),
                ),
            ),
        ));

        let mut ctx = uniquify::ProgramUniquifier {};
        let actual = ctx.fold(prog);
        assert_eq!(actual, expected);
    }
}
