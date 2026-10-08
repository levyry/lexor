#![allow(clippy::missing_errors_doc)]

pub mod core;
mod engineview;
mod graphred;
mod lambda;
pub mod tokenize;
pub mod statespace;

pub use engineview::EngineGraphNode;
pub use engineview::EngineGraphNodeKind;
pub use engineview::EngineView;
pub use tokenize::{ArgIndex, EngineToken, NodeRole};
pub use graphred::{ReductionError as SkiReductionError, ReductionStrategy as SkiReductionStrat};
pub use lambda::{LambdaEvalError, evaluate_lambda};
pub use lexor_core::de_bruijn::LambdaReductionStrategy;
pub use statespace::{StateSpace, generate_state_space};
