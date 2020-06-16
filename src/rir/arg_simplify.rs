use super::{Expr, ExprFolder};

pub struct ExprArgSimplifier {
    pub counter: u64,
}

impl ExprArgSimplifier {
    pub fn new(counter: u64) -> ExprArgSimplifier {
        ExprArgSimplifier { counter }
    }

    pub fn new_sym_name(&mut self) -> String {
        let name = format!("v{}", self.counter);
        self.counter += 1;
        name
    }
}

impl ExprFolder for ExprArgSimplifier {
    fn fold(&mut self, e: Box<Expr>) -> Box<Expr> {
        match *e {
            Expr::Read => e,   // No args so just return
            Expr::Lit(_) => e, // Primitive so no args so just return
            Expr::Neg(op) => {
                if is_complex_operand(&op) {
                    let new_sym_name = self.new_sym_name();
                    let folded_op = self.fold(op);
                    Expr::let_bind(
                        &new_sym_name,
                        folded_op,
                        Expr::neg(Expr::var(&new_sym_name)),
                    )
                } else {
                    Expr::neg(op)
                }
            }
            Expr::Add(op1, op2) => match (is_complex_operand(&op1), is_complex_operand(&op2)) {
                (true, true) => {
                    let new_sym_name1 = self.new_sym_name();
                    let folded_op1 = self.fold(op1);
                    let new_sym_name2 = self.new_sym_name();
                    let folded_op2 = self.fold(op2);
                    Expr::let_bind(
                        &new_sym_name1,
                        folded_op1,
                        Expr::let_bind(
                            &new_sym_name2,
                            folded_op2,
                            Expr::add(Expr::var(&new_sym_name1), Expr::var(&new_sym_name2)),
                        ),
                    )
                }
                (true, false) => {
                    let new_sym_name = self.new_sym_name();
                    let folded_op1 = self.fold(op1);
                    Expr::let_bind(
                        &new_sym_name,
                        folded_op1,
                        Expr::add(Expr::var(&new_sym_name), op2),
                    )
                }
                (false, true) => {
                    let new_sym_name = self.new_sym_name();
                    let folded_op2 = self.fold(op2);
                    Expr::let_bind(
                        &new_sym_name,
                        folded_op2,
                        Expr::add(op1, Expr::var(&new_sym_name)),
                    )
                }
                (false, false) => Expr::add(op1, op2),
            },
            Expr::Var(_) => e, // Return var
            Expr::Let(sym, e, body) => Box::new(Expr::Let(sym, self.fold(e), self.fold(body))), // Recurse down e and body
        }
    }
}

fn is_complex_operand(op: &Expr) -> bool {
    match *op {
        Expr::Lit(_) => false,
        Expr::Var(_) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expr, ExprFolder};
    use super::ExprArgSimplifier;

    #[test]
    fn already_simplified() {
        let expr = Expr::let_bind(
            "foo",
            Expr::let_bind(
                "bar",
                Expr::int(10),
                Expr::add(Expr::int(20), Expr::var("bar")),
            ),
            Expr::neg(Expr::var("foo")),
        );

        let mut ctx = ExprArgSimplifier::new(200_000);
        let actual = ctx.fold(expr.clone());
        assert_eq!(actual, expr);
    }

    #[test]
    fn simplify_neg_arg() {
        let expr = Expr::neg(Expr::read());
        let expected = Expr::let_bind("v200000", Expr::read(), Expr::neg(Expr::var("v200000")));

        let mut ctx = ExprArgSimplifier::new(200_000);
        let actual = ctx.fold(expr);
        assert_eq!(actual, expected);
    }

    #[test]
    fn simplify_add_args() {
        let expr = Expr::add(
            Expr::add(Expr::int(1), Expr::int(2)),
            Expr::add(
                Expr::add(Expr::int(3), Expr::read()),
                Expr::add(Expr::read(), Expr::int(4)),
            ),
        );

        let expected = Expr::let_bind(
            "v200000",
            Expr::add(Expr::int(1), Expr::int(2)),
            Expr::let_bind(
                "v200001",
                Expr::let_bind(
                    "v200002",
                    Expr::let_bind(
                        "v200003",
                        Expr::read(),
                        Expr::add(Expr::int(3), Expr::var("v200003")),
                    ),
                    Expr::let_bind(
                        "v200004",
                        Expr::let_bind(
                            "v200005",
                            Expr::read(),
                            Expr::add(Expr::var("v200005"), Expr::int(4)),
                        ),
                        Expr::add(Expr::var("v200002"), Expr::var("v200004")),
                    ),
                ),
                Expr::add(Expr::var("v200000"), Expr::var("v200001")),
            ),
        );

        let mut ctx = ExprArgSimplifier::new(200_000);
        let actual = ctx.fold(expr);
        assert_eq!(actual, expected);
    }
}
