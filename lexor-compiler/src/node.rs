slotmap::new_key_type! {
    /// A key to access Nodes in the AST Arena
    pub struct NodeKey;
}

#[rustfmt::skip]
/// A node in the AST
#[derive(Debug, Clone)]
pub enum Node {
    // Regular Combinators
    S, K, I, B, C,
    // Bulk combinators
    Sn(u8), Bn(u8), Cn(u8),
    // Graph structure
    App(NodeKey, NodeKey),
    Indirection(NodeKey),
}
