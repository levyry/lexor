#![allow(unused)]

use chumsky::prelude::*;

/// A recursive combinator structure that we parse into. Later, this gets
/// flattened into an Arena.
#[derive(Debug, Clone)]
pub enum CombRec {
    S,
    K,
    I,
    B,
    C,
    App(Box<Self>, Box<Self>),
}

///
/// # Errors
///
pub fn parse(input: &str) -> Result<CombRec, Vec<Rich<'_, char>>> {
    fn parser<'a>() -> impl Parser<'a, &'a str, CombRec, extra::Err<Rich<'a, char>>> {
        recursive(|expr| {
            let atom = choice((
                just('S').to(CombRec::S),
                just('K').to(CombRec::K),
                just('I').to(CombRec::I),
                just('B').to(CombRec::B),
                just('C').to(CombRec::C),
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
