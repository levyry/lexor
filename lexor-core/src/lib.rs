/*!
This crate provides several algorithms for parsing and converting between
different [Lambda calculus] and [Combinatory calculi] representations.

TODO: Finish once done with crate. Add example.

[Lambda calculus]: https://en.wikipedia.org/wiki/Lambda_calculus
[Combinator]: https://en.wikipedia.org/wiki/Combinatory_logic
*/

pub mod combinator;
pub mod de_bruijn;
pub mod kiselyov;
pub mod lambda;
pub mod parser;

pub use lambda::Lambda;
pub use parser::{ParseError, parse};
