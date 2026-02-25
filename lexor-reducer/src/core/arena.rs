use std::fmt::Debug;

use crate::core::node::{Node, NodeComb, NodeKey};
use lexor_parser::combinator::Combinator;
use slotmap::SlotMap;

// Abstract away the concrete arena impl
pub trait Arena: Default + Debug {
    fn presized(size: usize) -> Self;
    fn size(&self) -> usize;
    fn get(&self, key: NodeKey) -> Option<&Node>;
    fn insert(&mut self, value: Node) -> NodeKey;
    fn replace(&mut self, key: NodeKey, replacement: Node);
    fn remove(&mut self, key: NodeKey);

    fn get_arg(&self, key: NodeKey) -> Option<NodeKey> {
        match self.get(key) {
            Some(Node::App(_, arg)) => Some(*arg),
            _ => None,
        }
    }

    fn flatten(&mut self, ast: Combinator) -> NodeKey {
        match ast {
            Combinator::S => self.insert(Node::Comb(NodeComb::S)),
            Combinator::K => self.insert(Node::Comb(NodeComb::K)),
            Combinator::I => self.insert(Node::Comb(NodeComb::I)),
            Combinator::B => self.insert(Node::Comb(NodeComb::B)),
            Combinator::C => self.insert(Node::Comb(NodeComb::C)),
            Combinator::App(lhs, rhs) => {
                let l_key = self.flatten(*lhs);
                let r_key = self.flatten(*rhs);
                self.insert(Node::App(l_key, r_key))
            }
        }
    }
}

impl Arena for SlotMap<NodeKey, Node> {
    fn presized(size: usize) -> Self {
        Self::with_capacity_and_key(size)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, key: NodeKey) -> Option<&Node> {
        self.get(key)
    }

    fn insert(&mut self, value: Node) -> NodeKey {
        self.insert(value)
    }

    fn replace(&mut self, key: NodeKey, replacement: Node) {
        if let Some(current) = self.get_mut(key) {
            *current = replacement;
        }
    }

    fn remove(&mut self, key: NodeKey) {
        self.remove(key);
    }
}
