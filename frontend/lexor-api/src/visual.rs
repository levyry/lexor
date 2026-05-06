use core::fmt;

use lexor_reducer::{
    NodeRole,
    core::node::{NodeComb, NodeKey},
};
use serde::{Deserialize, Serialize};

use crate::map_comb_to_visual;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VisualComb {
    S,
    K,
    I,
    B,
    C,
}

impl fmt::Display for VisualComb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::S => "S",
            Self::K => "K",
            Self::I => "I",
            Self::B => "B",
            Self::C => "C",
        })
    }
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

impl From<NodeRole> for TokenStyle {
    fn from(value: NodeRole) -> Self {
        match value {
            NodeRole::Normal => Self::Normal,
            NodeRole::RedexHead(comb) => Self::RedexHead(map_comb_to_visual(comb)),
            NodeRole::RedexArg(comb, idx) => Self::RedexBody(map_comb_to_visual(comb), idx),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RenderToken {
    pub text: String,
    pub style: TokenStyle,
    pub node_key: Option<NodeKey>,
}
