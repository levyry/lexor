use std::collections::HashMap;

use lexor_api::{
    LambdaReductionStrategy, SkiReductionStrat,
    response::{GraphStep, ReductionStep},
    source_id::SourceKind,
};
use serde::{Deserialize, Serialize};

use crate::graph::LexorGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub lambda_strategy: LambdaReductionStrategy,
    pub ski_strategy: SkiReductionStrat,
    pub reduction_chain: Option<Vec<ReductionStep>>,
    pub reduction_graph: Option<Vec<GraphStep>>,
    pub active_graph_step: usize,
    pub lambda_output: Option<String>,
    pub ski_input: String,
    pub lambda_input: String,
    pub converted_lambda_output: Option<String>,
    pub error: Option<String>,
    pub last_edited_time: f64,

    #[serde(skip)]
    pub compiled_graphs: HashMap<usize, LexorGraph>,
}

impl Source {
    #[must_use]
    pub fn new(kind: SourceKind) -> Self {
        Self {
            kind,
            last_edited_time: 0.0,
            error: None,
            ski_input: String::from("SKISKI"),
            lambda_input: String::from("\\x.\\y.\\z.y(x z)"),
            ski_strategy: SkiReductionStrat::NormalForm,
            lambda_strategy: LambdaReductionStrategy::NormalOrder,
            reduction_chain: None,
            reduction_graph: None,
            active_graph_step: 0,
            lambda_output: None,
            compiled_graphs: HashMap::new(),
            converted_lambda_output: None,
        }
    }

    #[must_use]
    pub const fn is_at_step(&self, step: usize) -> bool {
        self.active_graph_step == step
    }

    pub const fn set_graph_step(&mut self, new_step: usize) {
        self.active_graph_step = new_step;
    }

    #[must_use]
    pub fn is_cached_for(&self, step: usize) -> bool {
        self.compiled_graphs.contains_key(&step)
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn remove_error(&mut self) {
        self.error = None;
    }
}
