use arrayvec::ArrayVec;
use lexor_core::combinator::Combinator;
use lower::saturating::math as saturating;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
};

const PRESIZE: usize = 10_000;

#[derive(Error, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum EngineError {
    #[error("tried reducing on an empty spine")]
    EmptySpine,
    #[error("tried reducing something that isn't a combinator")]
    BadReduce,
    #[error("{0}")]
    MissingKey(#[from] super::arena::ArenaError),
}

type Result<T> = core::result::Result<T, EngineError>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionState {
    #[default]
    Reduced,
    Halted,
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub root: NodeKey,
    pub subtree_root: NodeKey,
    pub spine: Vec<NodeKey>,
    pub arena: Arena,
}

impl Engine {
    #[must_use]
    pub fn from_tree(root: Combinator) -> Self {
        let mut arena = Arena::presized(PRESIZE);

        let root = arena.flatten(root);

        Self {
            root,
            arena,
            subtree_root: root,
            spine: vec![],
        }
    }

    fn resolve_indirection(&mut self, origin: NodeKey) -> NodeKey {
        let mut current = origin;

        while let Ok(&Node::Indirection(target)) = self.arena.get(current) {
            // Path compression
            if let Some(&parent_key) = self.spine.last()
                && let Ok(Node::App(_, r)) = self.arena.get(parent_key)
            {
                self.arena.replace(parent_key, Node::App(target, *r));
            } else {
                // Empty spine, parent is root
                self.subtree_root = target;
            }
            current = target;
        }

        current
    }

    pub(crate) fn unwind(&mut self, current: NodeKey) -> Result<()> {
        let mut current = current;
        // TODO: Rewrite this using a while loop so the control flow
        //       is less obfuscated.
        loop {
            match self.arena.get(current) {
                Ok(Node::App(lhs, _)) => {
                    self.spine.push(current);
                    current = *lhs;
                }
                Ok(Node::Comb(_)) => {
                    self.spine.push(current);
                    break;
                }

                Ok(Node::Indirection(_)) => {
                    current = self.resolve_indirection(current);
                }
                Err(err) => {
                    return Err(EngineError::MissingKey(err));
                }
            }
        }
        Ok(())
    }

    // SAFETY: Since we took `arg_count` amount of elements from the spine,
    //         `args` has the correct amount of elements in each match arm
    //         such that we never index out of bounds.
    #[allow(clippy::indexing_slicing)]
    pub(crate) fn reduce(&mut self) -> Result<ReductionState> {
        let redex_key = self.spine.pop().ok_or(EngineError::EmptySpine)?;

        let &Node::Comb(redex) = self.arena.get(redex_key).map_err(EngineError::MissingKey)? else {
            return Err(EngineError::BadReduce);
        };

        let arg_count = redex.arity();

        let spine_len = self.spine.len();

        if spine_len < arg_count {
            self.spine.push(redex_key);
            return Ok(ReductionState::Halted);
        }

        let drain_index = saturating! { spine_len - arg_count };

        let roots: ArrayVec<NodeKey, 3> = self.spine.drain(drain_index..spine_len).collect();

        // TODO: Look into refactoring this to be prettier maybe?
        let args: Result<ArrayVec<NodeKey, 3>> = roots
            .iter()
            .rev()
            .map(|&key| self.arena.get_arg(key))
            .try_fold(ArrayVec::<NodeKey, 3>::new(), |mut unrolled, result| {
                unrolled.push(result?);
                Ok(unrolled)
            });

        let args = args?;

        let result = match redex {
            NodeComb::I | NodeComb::K => Node::Indirection(args[0]),
            NodeComb::S => {
                let xz = self.arena.insert(Node::App(args[0], args[2]));
                let yz = self.arena.insert(Node::App(args[1], args[2]));
                Node::App(xz, yz)
            }
            NodeComb::B => {
                let yz = self.arena.insert(Node::App(args[1], args[2]));
                Node::App(args[0], yz)
            }
            NodeComb::C => {
                let xz = self.arena.insert(Node::App(args[0], args[2]));
                Node::App(xz, args[1])
            }
        };

        self.arena.replace(roots[0], result);

        Ok(ReductionState::Reduced)
    }
}
