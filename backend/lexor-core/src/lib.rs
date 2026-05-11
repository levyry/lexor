/*!
This crate provides several algorithms for parsing and converting between
different [Lambda calculus] and [Combinatory calculi] representations.

TODO: Finish once done with crate. Add example.

[Lambda calculus]: https://en.wikipedia.org/wiki/Lambda_calculus
[Combinator]: https://en.wikipedia.org/wiki/Combinatory_logic
*/
#![feature(iter_advance_by)]

pub mod de_bruijn;
mod name_generator;

pub mod lambda {
    /*!
    This module provides a definition for a Lambda calculus term in the
    standard representation. This is only used for parsing.

    TODO: Finish docs, add example
    */

    use core::fmt;

    use crate::{combinator::Combinator, de_bruijn::DeBruijn, name_generator::NameGenerator};

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
                Self::App(lhs, rhs) if let Self::App(_, _) = **rhs => write!(f, "({lhs})({rhs})"),
                Self::App(lhs, rhs) => write!(f, "({lhs}){rhs}"),
            }
        }
    }

    impl TryFrom<DeBruijn> for Lambda {
        type Error = String;

        fn try_from(value: DeBruijn) -> Result<Self, Self::Error> {
            fn convert(
                db: DeBruijn,
                scope: &mut Vec<String>,
                alphabet: &mut impl Iterator<Item = String>,
            ) -> Result<Lambda, String> {
                match db {
                    DeBruijn::BVar(index) => {
                        let name = scope
                            .iter()
                            .rev()
                            .nth(index)
                            .ok_or_else(|| "De Bruijn index out of bounds".to_owned())?
                            .clone();

                        Ok(Lambda::Var(name))
                    }

                    DeBruijn::FVar(name) => Ok(Lambda::Var(name)),

                    DeBruijn::Abs(body) => {
                        let name = alphabet.next().ok_or("Ran out of names")?;

                        scope.push(name.clone());

                        let body_lambda_expr = convert(*body, scope, alphabet)?;

                        scope.pop();

                        Ok(Lambda::Abs(name, Box::new(body_lambda_expr)))
                    }
                    DeBruijn::App(lhs, rhs) => Ok(Lambda::App(
                        Box::new(convert(*lhs, scope, alphabet)?),
                        Box::new(convert(*rhs, scope, alphabet)?),
                    )),
                }
            }

            convert(value, &mut Vec::new(), &mut NameGenerator::new())
        }
    }

    impl From<Combinator> for Lambda {
        fn from(value: Combinator) -> Self {
            macro_rules! var {
                ($name:ident) => {
                    Lambda::Var(stringify!($name).to_string())
                };
            }

            macro_rules! abs {
                ($arg:ident, $body:expr) => {
                    Lambda::Abs(stringify!($arg).to_string(), Box::new($body))
                };
            }

            macro_rules! app {
                ($lhs:expr, $rhs:expr) => {
                    Lambda::App(Box::new($lhs), Box::new($rhs))
                };
            }

            match value {
                Combinator::S => abs!(
                    x,
                    abs!(
                        y,
                        abs!(z, app!(app!(var!(x), var!(z)), app!(var!(y), var!(z))))
                    )
                ),
                Combinator::K => abs!(x, abs!(y, var!(x))),
                Combinator::I => abs!(x, var!(x)),
                Combinator::B => abs!(x, abs!(y, abs!(z, app!(var!(x), app!(var!(y), var!(z)))))),
                Combinator::C => abs!(x, abs!(y, abs!(z, app!(app!(var!(x), var!(z)), var!(y))))),
                Combinator::App(lhs, rhs) => app!((*lhs).into(), (*rhs).into()),
            }
        }
    }
}

pub mod combinator {
    /*!
    This module provides a definition for a [`Combinator`]. This is only used
    for parsing.

    TODO: Finish docs, add example
    */

    use core::fmt;
    use std::ops::BitAnd;

    /// Represents a SKI combinator term. Supports free variables, and the
    /// following combinators: S, K, I, B, C
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Combinator {
        /// S combinator: Sxyz -> xz(yz)
        S,
        /// K combinator: Kxy -> x
        K,
        /// I combinator: Ix -> x
        I,
        /// B combinator: Bxyz -> x(yz)
        B,
        /// C combinator: Cxyz -> xzy
        C,
        /// The application of two terms, such as (KS)(KI)
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
                Self::App(lhs, rhs) if let Self::App(_, _) = **rhs => write!(f, "{lhs}({rhs})"),
                Self::App(lhs, rhs) => write!(f, "{lhs}{rhs}"),
            }
        }
    }
}
