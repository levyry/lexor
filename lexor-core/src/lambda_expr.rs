use crate::de_bruijn::DeBruijn;

use LambdaExpr::{Abs, App, Var};

#[derive(Clone, Debug, PartialEq)]
pub enum LambdaExpr {
    Var(String),
    Abs(String, Box<Self>),
    App(Box<Self>, Box<Self>),
}
impl LambdaExpr {
    #[allow(unused)]
    pub(crate) fn pretty_print(self) -> String {
        match self {
            Var(name) => name,
            Abs(name, lambda_expr) => format!("λ{}.{}", name, lambda_expr.pretty_print()),
            App(lhs, rhs) => format!("({})({})", lhs.pretty_print(), rhs.pretty_print()),
        }
    }
}

impl From<DeBruijn> for LambdaExpr {
    fn from(db: DeBruijn) -> Self {
        fn convert(db: DeBruijn, scope: &mut Vec<String>, counter: usize) -> LambdaExpr {
            match db {
                DeBruijn::BVar(index) => Var(scope
                    .iter()
                    .rev()
                    .nth(index)
                    .expect("De Bruijn index out of bounds")
                    .clone()),

                DeBruijn::FVar(name) => LambdaExpr::Var(name),
                DeBruijn::Abs(body) => {
                    let name = format!("x{counter}");
                    let new_counter = counter.saturating_add(1);

                    scope.push(name.clone());
                    let body_lambda_expr = convert(*body, scope, new_counter);
                    scope.pop();

                    LambdaExpr::Abs(name, Box::new(body_lambda_expr))
                }

                DeBruijn::App(lhs, rhs) => LambdaExpr::App(
                    Box::new(convert(*lhs, scope, counter)),
                    Box::new(convert(*rhs, scope, counter)),
                ),
            }
        }

        convert(db, &mut Vec::new(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de_bruijn::DeBruijn;

    macro_rules! var {
        ($s:expr) => {
            LambdaExpr::Var($s.to_string())
        };
    }
    macro_rules! abs {
        ($p:expr, $b:expr) => {
            LambdaExpr::Abs($p.to_string(), Box::new($b))
        };
    }
    macro_rules! app {
        ($l:expr, $r:expr) => {
            LambdaExpr::App(Box::new($l), Box::new($r))
        };
    }

    #[test]
    fn test_print_abs() {
        let expr = abs!("x", var!("x"));
        assert_eq!(expr.pretty_print(), "λx.x");
    }

    #[test]
    fn test_print_app() {
        let expr = app!(var!("a"), var!("b"));

        assert_eq!(expr.pretty_print(), "(a)(b)");
    }

    #[test]
    fn test_print_complex() {
        let expr = abs!("x", app!(var!("y"), var!("z")));
        assert_eq!(expr.pretty_print(), "λx.(y)(z)");
    }

    #[test]
    fn test_from_debruijn_identity() {
        let db = DeBruijn::Abs(Box::new(DeBruijn::BVar(0)));
        let lambda = LambdaExpr::from(db);

        if let LambdaExpr::Abs(name, body) = lambda {
            assert_eq!(name, "x0");
            assert_eq!(*body, LambdaExpr::Var("x0".to_string()));
        } else {
            panic!("Expected Abs");
        }
    }

    #[test]
    fn test_from_debruijn_k_combinator() {
        let db = DeBruijn::Abs(Box::new(DeBruijn::Abs(Box::new(DeBruijn::BVar(1)))));
        let lambda = LambdaExpr::from(db);

        assert_eq!(lambda.pretty_print(), "λx0.λx1.x0");
    }

    #[test]
    fn test_from_debruijn_free_vars() {
        let db = DeBruijn::FVar("z".to_string());
        let lambda = LambdaExpr::from(db);
        assert_eq!(lambda, LambdaExpr::Var("z".to_string()));
    }

    #[test]
    #[should_panic(expected = "De Bruijn index out of bounds")]
    fn test_from_debruijn_out_of_bounds_panic() {
        let db = DeBruijn::BVar(0);
        let _ = LambdaExpr::from(db);
    }
}
