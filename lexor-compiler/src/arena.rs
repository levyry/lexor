use std::fmt::Debug;

use slotmap::SlotMap;

use crate::{
    node::{Node, NodeKey},
    ski_parser::CombRec,
};

// Abstract away the concrete arena impl
pub trait Arena: Default + Debug {
    fn get(&self, key: NodeKey) -> Option<&Node>;
    fn insert(&mut self, value: Node) -> NodeKey;
    fn replace(&mut self, key: NodeKey, replacement: Node);

    fn get_arg(&self, key: NodeKey) -> Option<NodeKey> {
        match self.get(key) {
            Some(Node::App(_, arg)) => Some(*arg),
            _ => None,
        }
    }

    fn flatten(&mut self, ast: CombRec) -> NodeKey {
        match ast {
            CombRec::S => self.insert(Node::S),
            CombRec::K => self.insert(Node::K),
            CombRec::I => self.insert(Node::I),
            CombRec::B => self.insert(Node::B),
            CombRec::C => self.insert(Node::C),
            CombRec::App(lhs, rhs) => {
                let l_key = self.flatten(*lhs);
                let r_key = self.flatten(*rhs);
                self.insert(Node::App(l_key, r_key))
            }
        }
    }
}

impl Arena for SlotMap<NodeKey, Node> {
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
}
