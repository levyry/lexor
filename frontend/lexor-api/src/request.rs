use serde::{Deserialize, Serialize};

use crate::{SourceID, strategy::ApiStrategy};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerRequest {
    pub source_id: SourceID,
    pub strategy: ApiStrategy,
    pub state: WorkerRequestState,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkerRequestState {
    Ski {
        wants_steps: bool,
        wants_graph: bool,
    },
    Lambda {
        // TODO: remove if unneeded
        placeholder: bool,
    },
}
