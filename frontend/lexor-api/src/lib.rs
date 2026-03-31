use lexor_reducer::{EngineView, NF, NodeRole, ReductionStrat, core::node::NodeComb};
use serde::{Deserialize, Serialize};

use crate::visual::{RenderToken, TokenStyle, VisualComb};

pub mod visual;

pub type SourceID = usize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerRequest {
    pub source_id: SourceID,
    pub code: String,
    pub wants_steps: bool,
    pub wants_graph: bool,
}

// TODO: Figure out graph nodes data type
pub type NodeData = ();

pub type ReductionStep = Vec<RenderToken>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerResponse {
    pub source_id: usize,
    pub steps: Option<Vec<ReductionStep>>,
    pub graph_nodes: Option<Vec<NodeData>>,
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
    let mut graph: Vec<NodeData> = vec![];

    let reduction_result = NF::compute_with(&req.code, |view: EngineView| {
        if req.wants_steps {
            let mut step_tokens = vec![];

            view.traverse(|text, role, key| {
                let style = match role {
                    NodeRole::Normal => TokenStyle::Normal,
                    NodeRole::RedexHead(comb) => TokenStyle::RedexHead(map_comb_to_visual(comb)),
                    NodeRole::RedexArg(comb, idx) => {
                        TokenStyle::RedexBody(map_comb_to_visual(comb), idx)
                    }
                };

                step_tokens.push(RenderToken {
                    text: text.to_owned(),
                    style,
                    node_key: key,
                });
            });

            steps.push(step_tokens);
        }

        if req.wants_graph {
            graph.push(()); // TODO: implement
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
