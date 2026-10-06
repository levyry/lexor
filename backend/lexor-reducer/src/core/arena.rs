use crate::core::node::{Node, NodeComb, NodeKey};
use lexor_core::combinator::Combinator;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Deserialize, Serialize)]
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
        match self.get(key) {
            Ok(Node::App(_, arg)) => Ok(*arg),
            Ok(_) => Err(ArenaError::MissingArg),
            Err(e) => Err(e),
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
            Combinator::Primitive(p) => self.0.insert(Node::Prim(p)),
        }
    }

    pub(crate) fn get_raw(&self, key: NodeKey) -> Result<&Node> {
        self.0.get(key).ok_or(ArenaError::MissingKey)
    }

    pub(crate) fn resolve(&self, mut key: NodeKey) -> NodeKey {
        while let Ok(Node::Indirection(target)) = self.get_raw(key) {
            key = *target;
        }
        key
    }

    pub(crate) fn get(&self, key: NodeKey) -> Result<&Node> {
        self.get_raw(self.resolve(key))
    }

    pub(crate) fn insert(&mut self, value: Node) -> NodeKey {
        self.0.insert(value)
    }
}
