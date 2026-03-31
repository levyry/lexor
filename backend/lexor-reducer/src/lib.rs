//! A crate for efficiently parsing and reducing SKI combinator expressions.

pub mod core;
mod engineview;
mod graphred;

pub use engineview::ArgIndex;
pub use engineview::EngineView;
pub use engineview::NodeRole;
pub use graphred::{NF, ReductionError, ReductionStrat, WHNF};

mod seal {
    use crate::graphred;

    /// Seal traits so other crates can't implement anything that is sealed.
    #[allow(unused)]
    pub trait Sealed {}
    impl Sealed for graphred::NormalForm {}
    impl Sealed for graphred::WeakHeadNormalForm {}
}
