use lexor_reducer::core::node::NodeKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VisualComb {
    S,
    K,
    I,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TokenStyle {
    Normal,
    RedexHead(VisualComb),
    RedexBody(VisualComb, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RenderToken {
    pub text: String,
    pub style: TokenStyle,
    pub node_key: Option<NodeKey>,
}
