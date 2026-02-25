use crate::core::node::{Node, NodeComb, NodeKey};
use lexor_parser::combinator::Combinator;
use slotmap::SlotMap;

#[derive(Debug, Clone)]
pub struct Arena(SlotMap<NodeKey, Node>);

impl Arena {
    #[must_use] 
    pub fn presized(size: usize) -> Self {
        Self(SlotMap::with_capacity_and_key(size))
    }

    pub fn replace(&mut self, key: NodeKey, replacement: Node) {
        if let Some(current) = self.0.get_mut(key) {
            *current = replacement;
        }
    }

    #[must_use] 
    pub fn get_arg(&self, key: NodeKey) -> NodeKey {
        match self.0.get(key) {
            Some(Node::App(_, arg)) => *arg,
            Some(_) => unreachable!("Called get_arg on not a Node::App"),
            None => unreachable!("Missing key in get_arg"),
        }
    }

    pub fn flatten(&mut self, ast: Combinator) -> NodeKey {
        match ast {
            Combinator::S => self.0.insert(Node::Comb(NodeComb::S)),
            Combinator::K => self.0.insert(Node::Comb(NodeComb::K)),
            Combinator::I => self.0.insert(Node::Comb(NodeComb::I)),
            Combinator::B => self.0.insert(Node::Comb(NodeComb::B)),
            Combinator::C => self.0.insert(Node::Comb(NodeComb::C)),
            Combinator::App(lhs, rhs) => {
                let l_key = self.flatten(*lhs);
                let r_key = self.flatten(*rhs);
                self.0.insert(Node::App(l_key, r_key))
            }
        }
    }

    #[must_use] 
    pub fn get(&self, key: NodeKey) -> Option<&Node> {
        self.0.get(key)
    }

    pub fn insert(&mut self, value: Node) -> NodeKey {
        self.0.insert(value)
    }
}
