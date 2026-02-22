//! A crate for efficiently parsing and reducing SKI combinator expressions.

mod core;
pub mod graphred;
pub mod parser;

pub use graphred::{ReductionMachine, ReductionMode};
pub use parser::parse;
