/*!
This module provides a definition for a Lambda calculus term in the
standard representation.

TODO: Finish docs, add example
*/

use core::fmt;

use crate::de_bruijn::DeBruijn;

/// Represents an untyped lambda calculus term (not using De Bruijn indexing).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lambda {
    /// Variable
    Var(String),
    /// Abstraction
    Abs(String, Box<Self>),
    /// Application
    App(Box<Self>, Box<Self>),
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(name) => write!(f, "{name}"),
            Self::Abs(name, lambda_expr) => write!(f, "λ{name}.{lambda_expr}"),
            // TODO: Rewrite once if let guards land in stable/nightly
            Self::App(lhs, rhs) => {
                if let Self::App(_, _) = **rhs {
                    write!(f, "({lhs})({rhs})")
                } else {
                    write!(f, "({lhs}){rhs}")
                }
            }
        }
    }
}

impl From<Lambda> for DeBruijn {
    fn from(expr: Lambda) -> Self {
        fn convert(expr: Lambda, scope: &mut Vec<String>) -> DeBruijn {
            match expr {
                Lambda::Var(name) => scope
                    .iter()
                    .rev()
                    .position(|n| n == &name)
                    .map_or(DeBruijn::FVar(name), DeBruijn::BVar),

                Lambda::Abs(arg, body) => {
                    scope.push(arg);
                    let body_de_bruijn = convert(*body, scope);
                    scope.pop();
                    DeBruijn::Abs(Box::new(body_de_bruijn))
                }

                Lambda::App(lhs, rhs) => DeBruijn::App(
                    Box::new(convert(*lhs, scope)),
                    Box::new(convert(*rhs, scope)),
                ),
            }
        }

        convert(expr, &mut Vec::new())
    }
}
