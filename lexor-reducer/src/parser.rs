use chumsky::prelude::*;

/// A recursive combinator structure that the parser returns. You can feed this
/// to [`crate::graphred::ReductionMachine::from_tree`] to initialize a [`crate::graphred::ReductionMachine`].
#[derive(Debug, Clone)]
pub enum CombRec {
    S,
    K,
    I,
    B,
    C,
    App(Box<Self>, Box<Self>),
}

/// Parses a given SKI term into a [`CombRec`]. Returns the root of the AST.
///
/// # Errors
///
/// If the given `input` isn't a valid SKI term (unsupported combinators,
/// mismatching parens, unknown characters).
///
/// # Example
///
/// ```
/// use lexor_reducer::{parse, CombRec};
///
/// let ski_term = "IKSKSSKISKSSSIK";
/// let tree_root = parse(ski_term).unwrap();
///
/// assert!(matches!(tree_root, CombRec::App(_, _)));
/// ```
///
pub fn parse(input: &str) -> Result<CombRec, Vec<Rich<'_, char>>> {
    fn parser<'a>() -> impl Parser<'a, &'a str, CombRec, extra::Err<Rich<'a, char>>> {
        recursive(|expr| {
            let atom = choice((
                just('S').or(just('s')).to(CombRec::S),
                just('K').or(just('k')).to(CombRec::K),
                just('I').or(just('i')).to(CombRec::I),
                just('B').or(just('b')).to(CombRec::B),
                just('C').or(just('c')).to(CombRec::C),
                expr.delimited_by(just('(').or(just('[')), just(')').or(just(']'))),
            ))
            .padded();

            atom.clone().foldl(atom.repeated(), |lhs, rhs| {
                CombRec::App(Box::new(lhs), Box::new(rhs))
            })
        })
        .then_ignore(end())
    }

    parser().parse(input).into_result()
}
