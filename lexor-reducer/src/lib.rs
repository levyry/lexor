//! A crate for efficiently parsing and reducing SKI combinator expressions.

mod core;
mod engineview;
mod graphred;
mod parser;

pub use engineview::EngineView;
pub use graphred::{NF, ReductionStrat, WHNF};
pub use parser::parse;

mod seal {
    use crate::graphred;

    /// Seal traits so other crates can't implement anything that is sealed.
    pub trait Sealed {}
    impl Sealed for graphred::NormalForm {}
    impl Sealed for graphred::WeakHeadNormalForm {}
}
