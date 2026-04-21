use serde::{Deserialize, Serialize};

use crate::visual::VisualComb;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApiGraphNodeKind {
    App,
    Comb(VisualComb),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeData {
    pub id: usize,
    pub kind: ApiGraphNodeKind,
    pub children: Vec<usize>,
}
