use serde::{Deserialize, Serialize};

use crate::{SourceID, strategy::ApiStrategy};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductionRequest {
    pub source_id: SourceID,
    pub strategy: ApiStrategy,
    pub state: ReductionRequestState,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReductionRequestState {
    Ski {
        wants_steps: bool,
        wants_graph: bool,
    },
    Lambda {
        // TODO: remove if unneeded
        placeholder: bool,
    },
}
