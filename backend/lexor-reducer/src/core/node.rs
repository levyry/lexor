use core::fmt;

slotmap::new_key_type! {
    /// A key to access Nodes in the AST Arena
    pub struct NodeKey;
}

/// A node in the flat AST
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Node {
    Comb(NodeComb),
    // Graph structure
    App(NodeKey, NodeKey),
    Indirection(NodeKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeComb {
    S,
    K,
    I,
    B,
    C,
}

impl NodeComb {
    #[must_use]
    pub const fn arity(&self) -> usize {
        match *self {
            Self::I => 1,
            Self::K => 2,
            Self::S | Self::C | Self::B => 3,
        }
    }
}

impl fmt::Display for NodeComb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::S => "S",
            Self::K => "K",
            Self::I => "I",
            Self::B => "B",
            Self::C => "C",
        })
    }
}
