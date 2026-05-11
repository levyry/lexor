use crate::{
    graph::{ApiGraphNodeKind, NodeData},
    request::{ReductionRequest, ReductionRequestState},
    response::{GraphStep, ReductionResponse, ReductionResponseState, ReductionStep},
    visual::RenderToken,
};
use lexor_reducer::{
    EngineGraphNodeKind::{self},
    EngineView, NF, ReductionStrat, evaluate_lambda,
};

pub mod graph;
pub mod request;
pub mod response;
pub mod source_id;
pub mod strategy;
pub mod visual;

pub use lexor_reducer::LambdaReductionStrategy;
use serde::{Deserialize, Serialize};
pub use source_id::SourceID;
pub use strategy::ApiStrategy;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "type")]
pub enum WorkerTask {
    Reduction(ReductionRequest),
    Conversion { source_id: SourceID, input: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "type")]
pub enum WorkerResult {
    Reduction(ReductionResponse),
    Conversion { source_id: SourceID, result: String },
}

#[must_use]
pub fn reduce_expression(req: &ReductionRequest) -> ReductionResponse {
    if let ReductionRequestState::Ski {
        wants_graph,
        wants_steps,
    } = req.state
    {
        if req.input.is_empty() {
            return ReductionResponse {
                source_id: req.source_id,
                state: response::ReductionResponseState::Ski {
                    steps: wants_steps.then_some(vec![]),
                    graph_nodes: wants_graph.then_some(vec![]),
                },
                error: None,
            };
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
                let engine_graph = view.extract_graph();

                let api_graph: Vec<NodeData> = engine_graph
                    .into_iter()
                    .map(|node| {
                        let kind = match node.kind {
                            EngineGraphNodeKind::App => ApiGraphNodeKind::App,
                            EngineGraphNodeKind::Comb(node_comb) => {
                                ApiGraphNodeKind::Comb(node_comb.into())
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
            Ok(()) => ReductionResponse {
                source_id: req.source_id,
                state: ReductionResponseState::Ski {
                    steps: wants_steps.then_some(steps),
                    graph_nodes: wants_graph.then_some(graph),
                },
                error: None,
            },
            Err(err) => ReductionResponse {
                source_id: req.source_id,
                state: ReductionResponseState::Ski {
                    steps: None,
                    graph_nodes: None,
                },
                error: Some(err.to_string()),
            },
        }
    } else if let ApiStrategy::Lambda(strat) = req.strategy {
        if req.input.is_empty() {
            return ReductionResponse {
                source_id: req.source_id,
                state: response::ReductionResponseState::Lambda {
                    output: Some(String::new()),
                },
                error: None,
            };
        }

        let result = evaluate_lambda(&req.input, strat);

        match result {
            Ok(str) => ReductionResponse {
                source_id: req.source_id,
                state: ReductionResponseState::Lambda { output: Some(str) },
                error: None,
            },
            Err(err) => ReductionResponse {
                source_id: req.source_id,
                state: ReductionResponseState::Lambda { output: None },
                error: Some(err.to_string()),
            },
        }
    } else {
        unreachable!("WorkerRequestState corrupted");
    }
}

pub fn convert_ski_to_lambda_string(input: &str) -> Result<String, String> {
    lexor_convert::convert_ski_to_lambda(input)
}
