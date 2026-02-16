#![allow(unused)]

use std::vec;

use crate::lambda_expr::LambdaExpr;
use DeBruijn::{Abs, App, BVar, FVar};

/// Locally nameless representation
///
/// Based on: <https://chargueraud.org/research/2009/ln/main.pdf>
#[derive(Debug, Clone, PartialEq)]
pub enum DeBruijn {
    BVar(usize),
    FVar(String),
    Abs(Box<Self>),
    App(Box<Self>, Box<Self>),
}

impl DeBruijn {
    fn var_opening(self, index: usize, name: String) -> Self {
        self.beta_reduce(index, FVar(name))
    }

    fn var_closing(self, name: String, index: usize) -> Self {
        match self {
            BVar(_) => self,

            FVar(ref x) => {
                if *x == name {
                    BVar(index)
                } else {
                    self
                }
            }

            Abs(body) => Abs(Box::new(body.var_closing(name, index.saturating_add(1)))),

            App(lhs, rhs) => App(
                Box::new(lhs.var_closing(name.clone(), index)),
                Box::new(rhs.var_closing(name, index)),
            ),
        }
    }

    fn is_closed_at(&self, limit: usize) -> bool {
        match self {
            BVar(k) => *k < limit,
            FVar(_) => true,
            Abs(body) => body.is_closed_at(limit.saturating_add(1)),
            App(lhs, rhs) => lhs.is_closed_at(limit) && rhs.is_closed_at(limit),
        }
    }

    fn is_closed(&self) -> bool {
        self.is_closed_at(0)
    }

    fn collect_free(self, acc: Vec<Self>) -> Vec<String> {
        match self {
            BVar(_) => vec![],
            FVar(name) => vec![name],
            Abs(body) => body.collect_free(acc),
            App(lhs, rhs) => {
                let new_lhs = lhs.collect_free(acc.clone());
                let new_rhs = rhs.collect_free(acc);

                let mut result: Vec<String> = vec![];

                for var in &new_lhs {
                    result.push(var.to_owned());
                }

                for var in &new_rhs {
                    result.push(var.to_owned());
                }

                result
            }
        }
    }

    fn substitute(self, original: String, replacement: &Self) -> Self {
        match self {
            BVar(_) => self,

            FVar(ref name) => {
                if *name == original {
                    replacement.clone()
                } else {
                    self
                }
            }

            Abs(body) => Abs(Box::new(body.substitute(original, replacement))),

            App(lhs, rhs) => App(
                Box::new(lhs.substitute(original.clone(), &replacement.clone())),
                Box::new(rhs.substitute(original, &replacement.clone())),
            ),
        }
    }

    pub fn beta_reduce(self, index: usize, other: Self) -> Self {
        match self {
            BVar(k) => {
                if k == index {
                    other
                } else {
                    self
                }
            }

            FVar(_) => self,

            Abs(body) => Abs(Box::new(body.beta_reduce(index.saturating_add(1), other))),

            App(lhs, rhs) => App(
                Box::new(lhs.beta_reduce(index, other.clone())),
                Box::new(rhs.beta_reduce(index, other)),
            ),
        }
    }
}

impl From<LambdaExpr> for DeBruijn {
    fn from(expr: LambdaExpr) -> Self {
        fn convert(expr: LambdaExpr, scope: &mut Vec<String>) -> DeBruijn {
            match expr {
                LambdaExpr::Var(name) => scope
                    .iter()
                    .rev()
                    .position(|n| n == &name)
                    .map_or(DeBruijn::FVar(name), DeBruijn::BVar),

                LambdaExpr::Abs(arg, body) => {
                    scope.push(arg);
                    let body_de_bruijn = convert(*body, scope);
                    scope.pop();
                    DeBruijn::Abs(Box::new(body_de_bruijn))
                }

                LambdaExpr::App(lhs, rhs) => DeBruijn::App(
                    Box::new(convert(*lhs, scope)),
                    Box::new(convert(*rhs, scope)),
                ),
            }
        }

        convert(expr, &mut Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- From<LambdaExpr> Tests ---

    #[test]
    fn test_from_lambda_identity() {
        let expr = LambdaExpr::Abs("x".to_string(), Box::new(LambdaExpr::Var("x".to_string())));
        let db = DeBruijn::from(expr);
        assert_eq!(db, DeBruijn::Abs(Box::new(DeBruijn::BVar(0))));
    }

    #[test]
    fn test_from_lambda_shadowing() {
        let expr = LambdaExpr::Abs(
            "x".to_string(),
            Box::new(LambdaExpr::Abs(
                "x".to_string(),
                Box::new(LambdaExpr::Var("x".to_string())),
            )),
        );

        let db = DeBruijn::from(expr);
        assert_eq!(
            db,
            DeBruijn::Abs(Box::new(DeBruijn::Abs(Box::new(DeBruijn::BVar(0)))))
        );
    }

    #[test]
    fn test_from_lambda_free_vars() {
        let expr = LambdaExpr::Abs("x".to_string(), Box::new(LambdaExpr::Var("y".to_string())));

        let db = DeBruijn::from(expr);
        assert_eq!(db, DeBruijn::Abs(Box::new(DeBruijn::FVar("y".to_string()))));
    }

    // --- Impl Method Tests ---

    #[test]
    fn test_var_opening() {
        let t = DeBruijn::BVar(0);
        assert_eq!(
            t.var_opening(0, "x".to_string()),
            DeBruijn::FVar("x".to_string())
        );

        let t = DeBruijn::Abs(Box::new(DeBruijn::BVar(1)));
        assert_eq!(
            t.var_opening(0, "x".to_string()),
            DeBruijn::Abs(Box::new(DeBruijn::FVar("x".to_string())))
        );
    }

    #[test]
    fn test_var_closing() {
        let t = DeBruijn::FVar("x".to_string());
        assert_eq!(t.var_closing("x".to_string(), 0), DeBruijn::BVar(0));

        let t = DeBruijn::Abs(Box::new(DeBruijn::FVar("x".to_string())));
        assert_eq!(
            t.var_closing("x".to_string(), 0),
            DeBruijn::Abs(Box::new(DeBruijn::BVar(1)))
        );
    }

    #[test]
    fn test_is_closed() {
        assert!(!DeBruijn::BVar(0).is_closed());

        assert!(DeBruijn::Abs(Box::new(DeBruijn::BVar(0))).is_closed());

        assert!(!DeBruijn::Abs(Box::new(DeBruijn::BVar(1))).is_closed());
    }

    #[test]
    fn test_collect_free() {
        let t = DeBruijn::App(
            Box::new(DeBruijn::FVar("x".to_string())),
            Box::new(DeBruijn::FVar("y".to_string())),
        );
        let free = t.collect_free(vec![]);
        assert_eq!(free, vec!["x", "y"]);

        let t = DeBruijn::Abs(Box::new(DeBruijn::FVar("x".to_string())));
        assert_eq!(t.collect_free(vec![]), vec!["x"]);

        let t = DeBruijn::Abs(Box::new(DeBruijn::BVar(0)));
        assert!(t.collect_free(vec![]).is_empty());
    }

    #[test]
    fn test_substitute() {
        let t = DeBruijn::App(
            Box::new(DeBruijn::FVar("x".to_string())),
            Box::new(DeBruijn::FVar("y".to_string())),
        );
        let replacement = DeBruijn::FVar("z".to_string());
        let res = t.substitute("x".to_string(), &replacement);

        assert_eq!(
            res,
            DeBruijn::App(
                Box::new(DeBruijn::FVar("z".to_string())),
                Box::new(DeBruijn::FVar("y".to_string()))
            )
        );
    }

    #[test]
    fn test_beta_reduce() {
        let t = DeBruijn::BVar(0);
        let sub = DeBruijn::FVar("A".to_string());
        assert_eq!(t.beta_reduce(0, sub), DeBruijn::FVar("A".to_string()));

        let t = DeBruijn::Abs(Box::new(DeBruijn::BVar(1)));
        let sub = DeBruijn::FVar("A".to_string());

        assert_eq!(
            t.beta_reduce(0, sub),
            DeBruijn::Abs(Box::new(DeBruijn::FVar("A".to_string())))
        );
    }
}
