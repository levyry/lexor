use std::fmt;

use crate::de_bruijn::DeBruijn;

use LambdaExpr::{Abs, App, Var};

#[derive(Clone, Debug, PartialEq)]
pub enum LambdaExpr {
    Var(String),
    Abs(String, Box<Self>),
    App(Box<Self>, Box<Self>),
}

impl fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn pretty_print(term: &LambdaExpr) -> String {
            match term {
                Var(name) => name.clone(),
                Abs(name, lambda_expr) => format!("λ{}.{}", name, pretty_print(lambda_expr)),
                App(lhs, rhs) => format!("({})({})", pretty_print(lhs), pretty_print(rhs)),
            }
        }

        write!(f, "{}", pretty_print(self))
    }
}

impl TryFrom<DeBruijn> for LambdaExpr {
    type Error = String;

    fn try_from(db: DeBruijn) -> Result<Self, Self::Error> {
        fn convert(
            db: DeBruijn,
            scope: &mut Vec<String>,
            counter: usize,
        ) -> Result<LambdaExpr, String> {
            match db {
                DeBruijn::BVar(index) => {
                    let name = scope
                        .iter()
                        .rev()
                        .nth(index)
                        .ok_or_else(|| "De Bruijn index out of bounds".to_string())?
                        .clone();

                    Ok(Var(name))
                }

                DeBruijn::FVar(name) => Ok(LambdaExpr::Var(name)),
                DeBruijn::Abs(body) => {
                    let name = format!("x{counter}");
                    let new_counter = counter.saturating_add(1);

                    scope.push(name.clone());
                    let body_lambda_expr = convert(*body, scope, new_counter)?;
                    scope.pop();

                    Ok(LambdaExpr::Abs(name, Box::new(body_lambda_expr)))
                }

                DeBruijn::App(lhs, rhs) => Ok(LambdaExpr::App(
                    Box::new(convert(*lhs, scope, counter)?),
                    Box::new(convert(*rhs, scope, counter)?),
                )),
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
        assert_eq!(expr.to_string(), "λx.x");
    }

    #[test]
    fn test_print_app() {
        let expr = app!(var!("a"), var!("b"));

        assert_eq!(expr.to_string(), "(a)(b)");
    }

    #[test]
    fn test_print_complex() {
        let expr = abs!("x", app!(var!("y"), var!("z")));
        assert_eq!(expr.to_string(), "λx.(y)(z)");
    }

    #[test]
    fn test_from_debruijn_identity() {
        let db = DeBruijn::Abs(Box::new(DeBruijn::BVar(0)));
        let lambda = LambdaExpr::try_from(db);

        match lambda {
            Ok(LambdaExpr::Abs(name, body)) => {
                assert_eq!(name, "x0");
                assert_eq!(*body, LambdaExpr::Var("x0".to_string()));
            }
            _ => assert!(false, "Expected Abs, got {lambda:?}"),
        }
    }

    #[test]
    fn test_from_debruijn_k_combinator() {
        let db = DeBruijn::Abs(Box::new(DeBruijn::Abs(Box::new(DeBruijn::BVar(1)))));
        match LambdaExpr::try_from(db) {
            Ok(lambda) => assert_eq!(lambda.to_string(), "λx0.λx1.x0"),
            Err(err) => assert!(false, "{err:?}"),
        }
    }

    #[test]
    fn test_from_debruijn_free_vars() {
        let db = DeBruijn::FVar("z".to_string());

        match LambdaExpr::try_from(db) {
            Ok(lambda) => assert_eq!(lambda, LambdaExpr::Var("z".to_string())),
            Err(err) => assert!(false, "{err:?}"),
        }
    }

    #[test]
    fn test_from_debruijn_out_of_bounds_panic() {
        let db = DeBruijn::BVar(0);
        assert!(LambdaExpr::try_from(db).is_err());
    }
}
