use chumsky::prelude::*;
use lexor_core::combinator::Combinator;

///
/// # Errors
///
/// If the given `input` isn't a valid SKI term (unsupported combinators,
/// mismatching parens, unknown characters).
///
/// # Example
///
/// ```
/// use lexor_reducer::parse;
/// use lexor_core::combinator::Combinator;
///
/// let ski_term = "IKSKSSKISKSSSIK";
/// let tree_root = parse(ski_term).unwrap();
///
/// assert!(matches!(tree_root, Combinator::App(_, _)));
/// ```
///
pub fn parse(input: &str) -> Result<Combinator, Vec<Rich<'_, char>>> {
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
