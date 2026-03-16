/*!
A crate for several parsing related data structures and parsers.
*/

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
        S,
        K,
        I,
        B,
        C,
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

pub mod lambda {
    /*!
    This module provides a definition for a Lambda calculus term in the
    standard representation. This is only used for parsing.

    TODO: Finish docs, add example
    */

    use core::fmt;

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
}

pub mod ski_parser {
    /*!
    This module provides a ski parser implemented with chumsky.
    */
    use crate::combinator::Combinator;
    use chumsky::prelude::*;

    ///
    /// # Errors
    ///
    /// If the given `input` isn't a valid SKI term (unsupported combinators,
    /// mismatching parens, unknown characters).
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_parser::{
    ///     combinator::Combinator,
    ///     ski_parser::chumsky_parse as parse
    /// };
    ///
    /// let ski_term = "IKSKSSKISKSSSIK";
    /// let tree_root = parse(ski_term).unwrap();
    ///
    /// assert!(matches!(tree_root, Combinator::App(_, _)));
    /// ```
    ///
    pub fn chumsky_parse(input: &str) -> Result<Combinator, Vec<Rich<'_, char>>> {
        fn parser<'a>() -> impl Parser<'a, &'a str, Combinator, extra::Err<Rich<'a, char>>> {
            recursive(|expr| {
                let atom = choice((
                    just('S').or(just('s')).to(Combinator::S),
                    just('K').or(just('k')).to(Combinator::K),
                    just('I').or(just('i')).to(Combinator::I),
                    just('B').or(just('b')).to(Combinator::B),
                    just('C').or(just('c')).to(Combinator::C),
                    expr.delimited_by(just('(').or(just('[')), just(')').or(just(']'))),
                ))
                .padded();

                atom.clone().foldl(atom.repeated(), |lhs, rhs| {
                    Combinator::App(Box::new(lhs), Box::new(rhs))
                })
            })
            .then_ignore(end())
        }

        parser().parse(input).into_result()
    }
}

pub mod lambda_parser {
    use crate::lambda::Lambda;
    use chumsky::extra;
    use chumsky::prelude::*;

    ///
    /// # Errors
    ///
    pub fn chumsky_parse(input: &str) -> Result<Lambda, Vec<Rich<'_, char>>> {
        fn parser<'a>() -> impl Parser<'a, &'a str, Lambda, extra::Err<Rich<'a, char>>> {
            recursive(|expr| {
                let var = text::ident()
                    .map(|s: &str| Lambda::Var(s.to_owned()))
                    .padded();

                let atom = var
                    .or(expr.clone().delimited_by(just('('), just(')')))
                    .padded();

                let app = atom.clone().foldl(atom.repeated(), |lhs, rhs| {
                    Lambda::App(Box::new(lhs), Box::new(rhs))
                });

                let lambda = just('λ')
                    .or(just('\\'))
                    .ignore_then(text::ident().padded())
                    .then_ignore(just('.'))
                    .then(expr.padded())
                    .map(|(arg, body): (&str, Lambda)| Lambda::Abs(arg.to_owned(), Box::new(body)));

                lambda.or(app)
            })
            .then_ignore(end())
        }

        parser().parse(input).into_result()
    }
}
