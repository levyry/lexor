use arrayvec::ArrayVec;
use lexor_parser::combinator::Combinator;
use lower::saturating::math as saturating;
use rootcause::{Report, option_ext::OptionExt, report};

use crate::core::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
};

const PRESIZE: usize = 10_000;

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

        while let Some(&Node::Indirection(target)) = self.arena.get(current) {
            // Path compression
            if let Some(&parent_key) = self.spine.last()
                && let Some(Node::App(_, r)) = self.arena.get(parent_key)
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

    pub fn unwind(&mut self, current: NodeKey) {
        let mut current = current;
        loop {
            match self.arena.get(current) {
                Some(Node::App(lhs, _)) => {
                    self.spine.push(current);
                    current = *lhs;
                }
                Some(Node::Comb(_)) => {
                    self.spine.push(current);
                    break;
                }

                Some(Node::Indirection(_)) => {
                    current = self.resolve_indirection(current);
                }
                None => {
                    unreachable!("Referenced key not in arena");
                }
            }
        }
    }
    pub fn reduce(&mut self) -> Result<ReductionState, Report> {
        let redex_key = self.spine.pop().context("Tried reducing on empty spine")?;

        let &Node::Comb(redex) = self.arena.get(redex_key).context("Missing key in arena")? else {
            return Err(report!("Tried reducing something that isn't a Node::Comb"));
        };

        let arg_count = match redex {
            NodeComb::I => 1,
            NodeComb::K => 2,
            NodeComb::S | NodeComb::C | NodeComb::B => 3,
        };

        let spine_len = self.spine.len();

        if spine_len < arg_count {
            self.spine.push(redex_key);
            return Ok(ReductionState::Halted);
        }

        let drain_index = saturating! { spine_len - arg_count };

        let roots: ArrayVec<NodeKey, 3> = self.spine.drain(drain_index..spine_len).collect();

        let args: ArrayVec<NodeKey, 3> = roots
            .iter()
            .rev()
            .map(|&key| self.arena.get_arg(key))
            .collect();

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
