/*!
This crate provides several algorithms for parsing and converting between
different [Lambda calculus] and [Combinatory calculi] representations.

TODO: Finish once done with crate. Add example.

[Lambda calculus]: https://en.wikipedia.org/wiki/Lambda_calculus
[Combinator]: https://en.wikipedia.org/wiki/Combinatory_logic
*/

pub mod de_bruijn;

pub mod lambda {
    /*!
    This module provides a definition for a Lambda calculus term in the
    standard representation. This is only used for parsing.

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
                counter: usize,
                alphabet: &mut std::iter::Chain<
                    std::ops::RangeInclusive<char>,
                    std::ops::RangeInclusive<char>,
                >,
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
                        let new_counter = counter.saturating_add(1);

                        scope.push(name.to_string());

                        let body_lambda_expr = convert(*body, scope, new_counter, alphabet)?;

                        scope.pop();

                        Ok(Lambda::Abs(name.to_string(), Box::new(body_lambda_expr)))
                    }
                    DeBruijn::App(lhs, rhs) => Ok(Lambda::App(
                        Box::new(convert(*lhs, scope, counter, alphabet)?),
                        Box::new(convert(*rhs, scope, counter, alphabet)?),
                    )),
                }
            }

            convert(
                value,
                &mut Vec::<String>::new(),
                0,
                &mut ('a'..='z').chain('A'..='Z'),
            )
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
