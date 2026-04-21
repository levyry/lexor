use lexor_reducer::core::node::{NodeComb, NodeKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VisualComb {
    S,
    K,
    I,
    B,
    C,
}

impl From<NodeComb> for VisualComb {
    fn from(value: NodeComb) -> Self {
        match value {
            NodeComb::S => Self::S,
            NodeComb::K => Self::K,
            NodeComb::I => Self::I,
            NodeComb::B => Self::B,
            NodeComb::C => Self::C,
        }
    }
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
