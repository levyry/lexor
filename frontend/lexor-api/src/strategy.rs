use core::fmt;

use lexor_reducer::LambdaReductionStrategy;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize, Serialize, Copy)]
pub enum ApiStrategy {
    Ski(()),
    Lambda(LambdaReductionStrategy),
}

impl fmt::Display for ApiStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ski(()) => f.write_str("Unimplemented"),
            Self::Lambda(strat) => strat.fmt(f),
        }
    }
}
