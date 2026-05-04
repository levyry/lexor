use lexor_reducer::{EngineView, NF, ReductionStrat, core::node::NodeComb};
use serde::{Deserialize, Serialize};

use crate::{
    graph::NodeData,
    visual::{RenderToken, VisualComb},
};

pub mod graph;
pub mod source_id;
pub mod visual;

pub use source_id::SourceID;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerRequest {
    pub source_id: SourceID,
    pub code: String,
    pub wants_steps: bool,
    pub wants_graph: bool,
}

pub type ReductionStep = Vec<RenderToken>;
pub type GraphStep = Vec<NodeData>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerResponse {
    pub source_id: SourceID,
    pub steps: Option<Vec<ReductionStep>>,
    pub graph_nodes: Option<Vec<GraphStep>>,
}

#[must_use]
pub fn process_job(req_json: &str) -> String {
    let req: WorkerRequest = serde_json::from_str(req_json).expect("Invalid JSON recieved");

    if req.code.is_empty() {
        let response = WorkerResponse {
            source_id: req.source_id,
            steps: req.wants_steps.then_some(vec![]),
            graph_nodes: req.wants_graph.then_some(vec![]),
        };

        return serde_json::to_string(&response).expect("Response serialization error");
    }

    let mut steps: Vec<ReductionStep> = vec![];
    let mut graph: Vec<GraphStep> = vec![];

    let reduction_result = NF::compute_with(&req.code, |view: EngineView| {
        if req.wants_steps {
            let mut step_tokens = vec![];

            view.traverse(|text, role, key| {
                step_tokens.push(RenderToken {
                    text: text.to_owned(),
                    style: role.into(),
                    node_key: key,
                });
            });

            steps.push(step_tokens);
        }

        if req.wants_graph {
            // Fetch the backend graph
            let engine_graph = view.extract_graph();

            // Map it to the API graph
            let api_graph: Vec<NodeData> = engine_graph
                .into_iter()
                .map(|node| {
                    let kind = match node.kind {
                        lexor_reducer::EngineGraphNodeKind::App => graph::ApiGraphNodeKind::App,
                        lexor_reducer::EngineGraphNodeKind::Comb(node_comb) => {
                            graph::ApiGraphNodeKind::Comb(node_comb.into())
                        }
                    };

                    NodeData {
                        id: node.id,
                        kind,
                        children: node.children,
                    }
                })
                .collect();

            graph.push(api_graph);
        }
    });

    if let Err(_error) = reduction_result {
        todo!("Figure out error handling")
    } else {
        let response = WorkerResponse {
            source_id: req.source_id,
            steps: req.wants_steps.then_some(steps),
            graph_nodes: req.wants_graph.then_some(graph),
        };

        serde_json::to_string(&response).expect("Response serialization error")
    }
}

// Helper function to cleanly map between boundaries
const fn map_comb_to_visual(comb: NodeComb) -> VisualComb {
    match comb {
        NodeComb::S => VisualComb::S,
        NodeComb::K => VisualComb::K,
        NodeComb::I => VisualComb::I,
        NodeComb::B => VisualComb::B,
        NodeComb::C => VisualComb::C,
    }
}
