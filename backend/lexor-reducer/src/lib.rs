//! A crate for efficiently parsing and reducing SKI combinator expressions.

pub mod core;
mod engineview;
mod graphred;
mod lambda;

pub use engineview::ArgIndex;
pub use engineview::EngineGraphNode;
pub use engineview::EngineGraphNodeKind;
pub use engineview::EngineView;
pub use engineview::NodeRole;
pub use graphred::{ReductionError as SkiReductionError, ReductionStrategy as SkiReductionStrat};
pub use lambda::{LambdaEvalError, evaluate_lambda};
pub use lexor_core::de_bruijn::LambdaReductionStrategy;
