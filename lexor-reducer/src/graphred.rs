#![allow(unused)]

use core::fmt;
use std::vec;

use crate::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
    parser::CombRec,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ReductionState {
    #[default]
    Reducible,
    Saturated,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionMode {
    #[default]
    NormalForm,
    WeakHeadNormalForm,
}

#[derive(Debug, Clone)]
pub struct GraphReductionEngine<Arn: Arena> {
    root: NodeKey,
    subtree_root: NodeKey,
    spine: Vec<NodeKey>,
    arena: Arn,
    mode: ReductionMode,
}

impl<Arn> GraphReductionEngine<Arn>
where
    Arn: Arena,
{
    #[must_use]
    pub fn from_tree(root: CombRec) -> Self {
        let mut arena = Arn::presized(1000);
        let arena_root = arena.flatten(root);

        Self {
            root: arena_root,
            subtree_root: arena_root,
            spine: vec![],
            arena,
            mode: ReductionMode::default(),
        }
    }

    #[must_use]
    pub const fn set_mode(mut self, new_mode: ReductionMode) -> Self {
        self.mode = new_mode;
        self
    }

    pub fn start(&mut self) {
        match self.mode {
            ReductionMode::NormalForm => self.reduce_to_nf(),
            ReductionMode::WeakHeadNormalForm => self.reduce_to_whnf(),
        }
    }

    fn reduce_to_whnf(&mut self) {
        let mut current_redex_state = ReductionState::Reducible;

        while current_redex_state == ReductionState::Reducible {
            let current = if let Some(&end) = self.spine.last() {
                match self.arena.get(end) {
                    Some(Node::App(lhs, _)) => *lhs,
                    Some(_) => end,
                    None => unreachable!(),
                }
            } else {
                self.subtree_root
            };

            self.unwind(current);

            current_redex_state = self.reduce();
        }
    }

    fn reduce_to_nf(&mut self) {
        self.reduce_to_whnf();

        let _head = self.spine.pop();

        while let Some(&redex_root) = self.spine.last()
            && let Some(Node::App(_lhs, rhs)) = self.arena.get(redex_root)
        {
            // NOTE: We choose to work with a fresh, empty spine and restoring
            // the original later. This is probably suboptimal, and using the
            // original spine could be more performant, but the main issue with
            // that approach is that we have to keep track of the boundary
            // between the old and new spines (which might cost more in the
            // long run).
            //
            // For example, the length of the spine might be 4, with 2 of those
            // terms coming from the current "subspine". At this point, if the
            // head is an S combinator, it try (and succeed in) popping 3 terms
            // off the spine, even thought the current subspine only has 2
            // terms. This is incorrect reduction behavior. Addings checks
            // everywhere so that we never cross main/sub spine boundaries when
            // popping arguments might waste more CPU cycles than the current
            // impl anyway.
            let old_spine = std::mem::take(&mut self.spine);
            self.subtree_root = *rhs;

            self.start();

            self.spine = old_spine;
            self.spine.pop();
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

    fn unwind(&mut self, current: NodeKey) {
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
                None => unreachable!("Referenced key not in arena"),
            }
        }
    }

    fn reduce(&mut self) -> ReductionState {
        let Some(operator_key) = self.spine.pop() else {
            return ReductionState::Saturated;
        };

        let Some(redex_root) = self.spine.pop() else {
            self.spine.push(operator_key);
            return ReductionState::Saturated;
        };

        let operator_node = self.arena.get(operator_key);

        let result = match operator_node {
            Some(&Node::Comb(operator)) => match operator {
                NodeComb::S | NodeComb::B | NodeComb::C => {
                    self.reduce_three_args(operator, redex_root)
                }
                NodeComb::K => self.reduce_two_args(operator, redex_root),
                NodeComb::I => {
                    if let Some(x) = self.arena.get_arg(redex_root) {
                        self.arena.replace(redex_root, Node::Indirection(x));
                        ReductionState::Reducible
                    } else {
                        ReductionState::Saturated
                    }
                }
                NodeComb::Sn(_) => todo!(),
                NodeComb::Bn(_) => todo!(),
                NodeComb::Cn(_) => todo!(),
            },
            Some(x) => unreachable!("Tried reducing something that isn't a comb: {x:?}"),
            None => unreachable!(),
        };

        if matches!(result, ReductionState::Saturated) {
            self.spine.push(redex_root);
            self.spine.push(operator_key);
        }

        result
    }

    fn reduce_three_args(&mut self, operator: NodeComb, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Saturated;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            return ReductionState::Saturated;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(y) = self.arena.get_arg(parent)
            && let Some(z) = self.arena.get_arg(grandparent)
        {
            let result = match operator {
                NodeComb::S => {
                    let xz = self.arena.insert(Node::App(x, z));
                    let yz = self.arena.insert(Node::App(y, z));
                    Node::App(xz, yz)
                }
                NodeComb::B => {
                    let yz = self.arena.insert(Node::App(y, z));
                    Node::App(x, yz)
                }
                NodeComb::C => {
                    let xz = self.arena.insert(Node::App(x, z));
                    Node::App(xz, y)
                }
                _ => unreachable!("Tried reducing three args on a comb that doesn't take three"),
            };
            self.arena.replace(grandparent, result);
            ReductionState::Reducible
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            ReductionState::Saturated
        }
    }

    fn reduce_two_args(&mut self, operator: NodeComb, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Saturated;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(_y) = self.arena.get_arg(parent)
        {
            let result = match operator {
                NodeComb::K => Node::Indirection(x),
                _ => unreachable!("Tried reducing two args on a comb that doesn't take two"),
            };

            self.arena.replace(parent, result);
            ReductionState::Reducible
        } else {
            self.spine.push(parent);
            ReductionState::Saturated
        }
    }
}

impl<Arn> fmt::Display for GraphReductionEngine<Arn>
where
    Arn: Arena + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node(
            arena: &impl Arena,
            key: NodeKey,
            f: &mut fmt::Formatter<'_>,
            is_rhs_of_app: bool,
        ) -> fmt::Result {
            let mut current = key;
            while let Some(Node::Indirection(target)) = arena.get(current) {
                current = *target;
            }

            match arena.get(current) {
                Some(Node::App(lhs, rhs)) => {
                    if is_rhs_of_app {
                        write!(f, "(")?;
                    }

                    print_node(arena, *lhs, f, false)?;
                    // write!(f, " ")?;
                    print_node(arena, *rhs, f, true)?;

                    if is_rhs_of_app {
                        write!(f, ")")?;
                    }
                    Ok(())
                }
                Some(Node::Comb(x)) => write!(f, "{x}"),
                Some(Node::Indirection(_)) => unreachable!("Indirections resolved already"),
                None => unreachable!("Node not found in arena"),
            }
        }

        print_node(&self.arena, self.root, f, false)
    }
}
