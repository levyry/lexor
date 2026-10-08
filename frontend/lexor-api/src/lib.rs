use crate::{
    graph::{ApiGraphNodeKind, NodeData},
    request::{ReductionRequest, ReductionRequestState},
    response::{GraphStep, ReductionResponse, ReductionResponseState, ReductionStep},
    visual::RenderToken,
};
use lexor_core::Literal;
use lexor_reducer::{
    EngineGraphNodeKind::{self},
    EngineView, evaluate_lambda,
};

pub mod graph;
pub mod request;
pub mod response;
pub mod source_id;
pub mod strategy;
pub mod visual;

pub use lexor_reducer::{LambdaReductionStrategy, SkiReductionStrat};
use serde::{Deserialize, Serialize};
pub use source_id::SourceID;
pub use strategy::ApiStrategy;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "type")]
pub enum WorkerTask {
    Reduction(ReductionRequest),
    Conversion { source_id: SourceID, input: String },
    DesugarChurch { source_id: SourceID, input: String },
    StateSpace { source_id: SourceID, input: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "type")]
pub enum WorkerResult {
    Reduction(ReductionResponse),
    Conversion {
        source_id: SourceID,
        result: String,
    },
    DesugarChurch {
        source_id: SourceID,
        result: String,
    },
    StateSpace {
        source_id: SourceID,
        result: Result<response::ApiStateSpace, String>,
    },
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn reduce_expression(req: &ReductionRequest) -> ReductionResponse {
    if let ReductionRequestState::Ski {
        wants_graph,
        wants_steps,
        desugar_church,
    } = req.state
        && let ApiStrategy::Ski(strat) = req.strategy
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

        let effective_input = if desugar_church {
            lexor_parser::parse_ski(&req.input).map_or_else(
                |_| req.input.clone(),
                |ast| ast.desugar_church().to_string(),
            )
        } else {
            req.input.clone()
        };

        let mut steps: Vec<ReductionStep> = vec![];
        let mut graph: Vec<GraphStep> = vec![];

        let result = strat.compute_with(&effective_input, |view: EngineView| {
            if wants_steps {
                let step_tokens = view
                    .tokenize()
                    .into_iter()
                    .map(|t| RenderToken {
                        text: t.text,
                        style: t.role.into(),
                        node_key: t.key,
                    })
                    .collect();

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
                            EngineGraphNodeKind::Prim(Literal::Num(n)) => ApiGraphNodeKind::Num(n),
                            EngineGraphNodeKind::Prim(Literal::Op(op)) => {
                                ApiGraphNodeKind::Op(op.to_string())
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
        ReductionResponse {
            source_id: req.source_id,
            state: ReductionResponseState::Lambda { output: None },
            error: Some("WorkerRequestState corrupted".to_owned()),
        }
    }
}

#[allow(clippy::missing_errors_doc)]
#[must_use]
#[wasm_bindgen]
pub fn desugar_ski_church(input: &str) -> String {
    lexor_parser::parse_ski(input)
        .map_or_else(|_| input.to_owned(), |ast| ast.desugar_church().to_string())
}

#[allow(clippy::missing_errors_doc)]
pub fn convert_ski_to_lambda_string(input: &str) -> Result<String, String> {
    lexor_convert::convert_ski_to_lambda(input)
}

#[must_use]
#[wasm_bindgen]
pub fn process_worker_job(request_json: &str) -> String {
    let task_result: Result<WorkerTask, serde_json::Error> = serde_json::from_str(request_json);

    let worker_result = match task_result {
        Ok(WorkerTask::Reduction(req)) => {
            let response = reduce_expression(&req);
            WorkerResult::Reduction(response)
        }
        Ok(WorkerTask::Conversion { source_id, input }) => {
            let result_str = convert_ski_to_lambda_string(&input).unwrap_or_else(|err| err);

            WorkerResult::Conversion {
                source_id,
                result: result_str,
            }
        }
        Ok(WorkerTask::DesugarChurch { source_id, input }) => {
            let result_str = desugar_ski_church(&input);

            WorkerResult::DesugarChurch {
                source_id,
                result: result_str,
            }
        }
        Ok(WorkerTask::StateSpace { source_id, input }) => {
            let result = lexor_reducer::generate_state_space(&input)
                .map_err(|e| e.to_string())
                .map(|space| {
                    use crate::response::{ApiStateSpace, ApiStateNode, ApiStateEdge};
                    ApiStateSpace {
                        nodes: space.nodes.into_iter().map(|n| ApiStateNode {
                            id: n.id,
                            expression: n.expression,
                        }).collect(),
                        edges: space.edges.into_iter().map(|e| ApiStateEdge {
                            from: e.from,
                            to: e.to,
                            is_normal_order: e.is_normal_order,
                            transition_tokens: e.transition_tokens.into_iter().map(|t| RenderToken {
                                text: t.text,
                                style: t.role.into(),
                                node_key: t.key,
                            }).collect(),
                        }).collect(),
                    }
                });

            WorkerResult::StateSpace { source_id, result }
        }
        Err(e) => {
            return format!(r#"{{"error": "Failed to parse worker task: {e}"}}"#);
        }
    };

    serde_json::to_string(&worker_result)
        .unwrap_or_else(|_| r#"{"error": "Serialization failed"}"#.to_owned())
}
