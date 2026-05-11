use crate::{
    graph::NodeData,
    request::{WorkerRequest, WorkerRequestState},
    response::{GraphStep, ReductionStep, WorkerResponse, WorkerResponseState},
    visual::{RenderToken, VisualComb},
};
use lexor_reducer::{EngineView, NF, ReductionStrat, core::node::NodeComb, eval};

pub mod graph;
pub mod request;
pub mod response;
pub mod source_id;
pub mod strategy;
pub mod visual;

pub use lexor_reducer::LambdaReductionStrategy;
pub use source_id::SourceID;
pub use strategy::ApiStrategy;

#[must_use]
pub fn process_job(req_json: &str) -> String {
    let req: WorkerRequest = serde_json::from_str(req_json).expect("Invalid JSON recieved");

    let response = if let WorkerRequestState::Ski {
        wants_graph,
        wants_steps,
    } = req.state
    {
        if req.input.is_empty() {
            let response = WorkerResponse {
                source_id: req.source_id,
                state: response::WorkerResponseState::Ski {
                    steps: wants_steps.then_some(vec![]),
                    graph_nodes: wants_graph.then_some(vec![]),
                },
                error: None,
            };

            return serde_json::to_string(&response).expect("Response serialization error");
        }

        let mut steps: Vec<ReductionStep> = vec![];
        let mut graph: Vec<GraphStep> = vec![];

        let result = NF::compute_with(&req.input, |view: EngineView| {
            if wants_steps {
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

            if wants_graph {
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

        match result {
            Ok(()) => WorkerResponse {
                source_id: req.source_id,
                state: WorkerResponseState::Ski {
                    steps: wants_steps.then_some(steps),
                    graph_nodes: wants_graph.then_some(graph),
                },
                error: None,
            },
            Err(err) => WorkerResponse {
                source_id: req.source_id,
                state: WorkerResponseState::Ski {
                    steps: None,
                    graph_nodes: None,
                },
                error: Some(err.to_string()),
            },
        }
    } else if let ApiStrategy::Lambda(strat) = req.strategy {
        if req.input.is_empty() {
            let response = WorkerResponse {
                source_id: req.source_id,
                state: response::WorkerResponseState::Lambda {
                    output: Some(String::new()),
                },
                error: None,
            };

            return serde_json::to_string(&response).expect("Response serialization error");
        }

        let result = eval(&req.input, strat);

        match result {
            Ok(str) => WorkerResponse {
                source_id: req.source_id,
                state: WorkerResponseState::Lambda { output: Some(str) },
                error: None,
            },
            Err(err) => WorkerResponse {
                source_id: req.source_id,
                state: WorkerResponseState::Lambda { output: None },
                error: Some(err.to_string()),
            },
        }
    } else {
        unreachable!("WorkerRequestState corrupted");
    };

    serde_json::to_string(&response).expect("Response serialization error")
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
