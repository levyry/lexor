use crate::core::node::{Node, NodeComb, NodeKey};
use lexor_core::combinator::Combinator;
use slotmap::SlotMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ArenaError {
    #[error("referenced key not in arena")]
    MissingKey,
    #[error("tried getting the argument of not a Node::App")]
    MissingArg,
}

type Result<T> = core::result::Result<T, ArenaError>;

#[derive(Debug, Clone)]
pub struct Arena(SlotMap<NodeKey, Node>);

impl Arena {
    #[must_use]
    pub(crate) fn presized(size: usize) -> Self {
        Self(SlotMap::with_capacity_and_key(size))
    }

    pub(crate) fn replace(&mut self, key: NodeKey, replacement: Node) {
        if let Some(current) = self.0.get_mut(key) {
            *current = replacement;
        }
    }

    pub(crate) fn get_arg(&self, key: NodeKey) -> Result<NodeKey> {
        match self.0.get(key) {
            Some(Node::App(_, arg)) => Ok(*arg),
            Some(_) => Err(ArenaError::MissingArg),
            None => Err(ArenaError::MissingKey),
        }
    }

    pub(crate) fn flatten(&mut self, ast: Combinator) -> NodeKey {
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

    pub(crate) fn get(&self, key: NodeKey) -> Result<&Node> {
        self.0.get(key).ok_or(ArenaError::MissingKey)
    }

    pub(crate) fn insert(&mut self, value: Node) -> NodeKey {
        self.0.insert(value)
    }
}
