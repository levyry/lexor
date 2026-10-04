use serde::{Deserialize, Serialize};

use crate::visual::VisualComb;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApiGraphNodeKind {
    App,
    Comb(VisualComb),
    Num(i64),
    Op(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeData {
    pub id: u64,
    pub kind: ApiGraphNodeKind,
    pub children: Vec<u64>,
}
