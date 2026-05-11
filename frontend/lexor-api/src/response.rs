use serde::{Deserialize, Serialize};

use crate::{SourceID, graph::NodeData, visual::RenderToken};

pub type ReductionStep = Vec<RenderToken>;
pub type GraphStep = Vec<NodeData>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerResponse {
    pub source_id: SourceID,
    pub state: WorkerResponseState,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkerResponseState {
    Ski {
        steps: Option<Vec<ReductionStep>>,
        graph_nodes: Option<Vec<GraphStep>>,
    },
    Lambda {
        output: Option<String>,
    },
}
