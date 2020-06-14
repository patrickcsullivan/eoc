use std::collections::HashMap;

/// Maintains state necessary for uniquify-ing the variable names in an AST.
pub struct Ctx {
    counter: u64,
    sym_table: HashMap<String, String>,
}

impl Ctx {
    const COUNTER_START: u64 = 12345;

    /// Returns a new Ctx.
    pub fn new() -> Ctx {
        Ctx {
            counter: Ctx::COUNTER_START,
            sym_table: HashMap::new(),
        }
    }

    /// Returns `true` if the symbol table contains the given source symbol.
    pub fn contains_src(&self, sym: &String) -> bool {
        self.sym_table.contains_key(sym)
    }

    /// Creates a generated symbol for the source symbol and inserts the symbols
    /// into the symbol table
    pub fn insert_sym(&mut self, sym: String) {
        if self.sym_table.get(&sym).is_none() {
            self.counter += 1;
            let gen = format!("v{}", self.counter);
            self.sym_table.insert(sym, gen);
        }
    }

    /// Returns the generated symbol that is associated with the given source
    /// symbol.
    pub fn get_gen(&self, sym: &String) -> Option<&String> {
        self.sym_table.get(sym)
    }
}
