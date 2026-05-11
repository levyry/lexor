/*!
Parsers for lexor TODO: rewrite
*/

pub mod ski_parser {
    /*!
    Parsers for lexor SKI TODO: rewrite
    */
    use chumsky::prelude::*;
    use lexor_core::combinator::Combinator;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    /// Parsing error
    #[derive(Error, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Deserialize, Serialize)]
    pub enum ParsingError {
        /// Chumsky
        #[error("Chumsky parsing error...")]
        ChumskyError,
    }

    ///
    /// # Errors
    ///
    /// If the given `input` isn't a valid SKI term (unsupported combinators,
    /// mismatching parens, unknown characters).
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_parser::ski_parser::chumsky_parse as parse;
    /// use lexor_core::combinator::Combinator;
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
    /*!
    Parsers for lexor Lambda TODO: rewrite
    */
    use chumsky::extra;
    use chumsky::prelude::*;
    use lexor_core::lambda::Lambda;

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
