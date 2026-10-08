use serde::{Deserialize, Serialize};

use crate::{SourceID, graph::NodeData, visual::RenderToken};

pub type ReductionStep = Vec<RenderToken>;
pub type GraphStep = Vec<NodeData>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductionResponse {
    pub source_id: SourceID,
    pub state: ReductionResponseState,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReductionResponseState {
    Ski {
        steps: Option<Vec<ReductionStep>>,
        graph_nodes: Option<Vec<GraphStep>>,
    },
    Lambda {
        output: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiStateNode {
    pub id: usize,
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiStateEdge {
    pub from: usize,
    pub to: usize,
    pub is_normal_order: bool,
    pub transition_tokens: Vec<RenderToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiStateSpace {
    pub nodes: Vec<ApiStateNode>,
    pub edges: Vec<ApiStateEdge>,
}
