use std::collections::HashMap;

use lexor_api::{GraphStep, ReductionStep};
use serde::{Deserialize, Serialize};

use crate::graph::LexorGraph;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SourceKind {
    Ski,
    Lambda,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub kind: SourceKind,
    pub contents: String,
    pub reduction_chain: Option<Vec<ReductionStep>>,
    pub reduction_graph: Option<Vec<GraphStep>>,
    pub last_edited_time: f64,
    pub active_graph_step: usize,

    #[serde(skip)]
    pub compiled_graphs: HashMap<usize, LexorGraph>,
}

impl Source {
    #[must_use]
    pub fn new(kind: SourceKind) -> Self {
        Self {
            kind,
            contents: String::new(),
            reduction_chain: None,
            reduction_graph: None,
            last_edited_time: 0.0,
            active_graph_step: 0,
            compiled_graphs: HashMap::new(),
        }
    }

    #[must_use]
    pub const fn is_at_step(&self, step: usize) -> bool {
        self.active_graph_step == step
    }

    #[must_use]
    pub fn is_cached_for(&self, step: usize) -> bool {
        self.compiled_graphs.contains_key(&step)
    }
}
