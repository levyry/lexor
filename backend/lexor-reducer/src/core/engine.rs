use arrayvec::ArrayVec;
use lexor_core::combinator::Combinator;
use lexor_core::literal::Literal;
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
                Ok(Node::Comb(_) | Node::Prim(_)) => {
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

    fn eval_num_whnf(&mut self, mut key: NodeKey) -> Result<Option<i64>> {
        while let Ok(Node::Indirection(target)) = self.arena.get(key) {
            key = *target;
        }
        match self.arena.get(key) {
            Ok(Node::Prim(Literal::Num(n))) => Ok(Some(*n)),
            Ok(Node::App(_, _)) => {
                // Reduce this sub-expression to WHNF
                let saved_spine = std::mem::take(&mut self.spine);
                let saved_root = self.subtree_root;
                self.subtree_root = key;

                let mut prev = ReductionState::Reduced;
                while prev == ReductionState::Reduced {
                    let curr = if let Some(&end) = self.spine.last() {
                        match self.arena.get(end)? {
                            Node::App(lhs, _) => *lhs,
                            _ => end,
                        }
                    } else {
                        self.subtree_root
                    };
                    self.unwind(curr)?;
                    prev = self.reduce()?;
                }

                self.subtree_root = saved_root;
                self.spine = saved_spine;

                // After reducing to WHNF, check again
                while let Ok(Node::Indirection(target)) = self.arena.get(key) {
                    key = *target;
                }
                if let Ok(Node::Prim(Literal::Num(n))) = self.arena.get(key) {
                    Ok(Some(*n))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    // SAFETY: Since we took `arg_count` amount of elements from the spine,
    //         `args` has the correct amount of elements in each match arm
    //         such that we never index out of bounds.
    #[allow(clippy::indexing_slicing)]
    pub(crate) fn reduce(&mut self) -> Result<ReductionState> {
        let redex_key = self.spine.pop().ok_or(EngineError::EmptySpine)?;

        let head_node = *self.arena.get(redex_key).map_err(EngineError::MissingKey)?;

        match head_node {
            Node::Comb(redex) => {
                let arg_count = redex.arity();
                let spine_len = self.spine.len();

                if spine_len < arg_count {
                    self.spine.push(redex_key);
                    return Ok(ReductionState::Halted);
                }

                let drain_index = spine_len.saturating_sub(arg_count);
                let roots: ArrayVec<NodeKey, 3> =
                    self.spine.drain(drain_index..spine_len).collect();

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
            Node::Prim(Literal::Op(op)) => {
                let arg_count = op.arity();
                let spine_len = self.spine.len();

                if spine_len < arg_count {
                    self.spine.push(redex_key);
                    return Ok(ReductionState::Halted);
                }

                let app1_index = spine_len.saturating_sub(1);
                let app0_index = spine_len.saturating_sub(2);
                let app1_key = self.spine[app1_index];
                let app0_key = self.spine[app0_index];
                let arg0_key = self.arena.get_arg(app1_key)?;
                let arg1_key = self.arena.get_arg(app0_key)?;

                let num0 = self.eval_num_whnf(arg0_key)?;
                let num1 = self.eval_num_whnf(arg1_key)?;

                if let (Some(n0), Some(n1)) = (num0, num1)
                    && let Some(res) = op.apply(n0, n1)
                {
                    let drain_index = spine_len.saturating_sub(arg_count);
                    let roots: ArrayVec<NodeKey, 3> =
                        self.spine.drain(drain_index..spine_len).collect();
                    self.arena.replace(roots[0], Node::Prim(Literal::Num(res)));
                    return Ok(ReductionState::Reduced);
                }

                self.spine.push(redex_key);
                Ok(ReductionState::Halted)
            }
            _ => {
                self.spine.push(redex_key);
                Ok(ReductionState::Halted)
            }
        }
    }
}
