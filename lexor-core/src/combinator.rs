use core::fmt;
use std::ops::BitAnd;

/// Represents a SKI combinator term. Supports free variables, and the
/// following combinators: S, K, I, B, C
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Combinator {
    S,
    K,
    I,
    B,
    C,
    Var(String),
    App(Box<Self>, Box<Self>),
}

// Application operator for ease of describing complex terms
impl BitAnd for Combinator {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::App(Box::new(self), Box::new(rhs))
    }
}

impl fmt::Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S => write!(f, "S"),
            Self::K => write!(f, "K"),
            Self::I => write!(f, "I"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::Var(s) => write!(f, "{s}"),
            // TODO: Rewrite once if let guards land in stable/nightly
            Self::App(lhs, rhs) => {
                if let Self::App(_, _) = **rhs {
                    write!(f, "{lhs}({rhs})")
                } else {
                    write!(f, "{lhs}{rhs}")
                }
            }
        }
    }
}
