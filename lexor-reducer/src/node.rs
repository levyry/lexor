#![allow(unused)]

use core::fmt;

slotmap::new_key_type! {
    /// A key to access Nodes in the AST Arena
    pub struct NodeKey;
}

/// A node in the flat AST
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    Comb(NodeComb),
    // Graph structure
    App(NodeKey, NodeKey),
    Indirection(NodeKey),
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeComb {
    // Regular Combinators
    S, K, I, B, C,
    // Bulk combinators
    Sn(u8), Bn(u8), Cn(u8),
}

impl fmt::Display for NodeComb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S => write!(f, "S"),
            Self::K => write!(f, "K"),
            Self::I => write!(f, "I"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::Sn(n) => write!(f, "S{n}"),
            Self::Bn(n) => write!(f, "B{n}"),
            Self::Cn(n) => write!(f, "C{n}"),
        }
    }
}
