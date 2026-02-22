use core::fmt;
use lexor_core::combinator::Combinator;

use crate::{
    core::arena::Arena,
    core::node::{Node, NodeComb, NodeKey},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionState {
    #[default]
    Reduced,
    Halted,
}

#[derive(Debug, Clone)]
pub struct Engine<Arn: Arena> {
    pub root: NodeKey,
    pub subtree_root: NodeKey,
    pub spine: Vec<NodeKey>,
    pub arena: Arn,
}

impl<Arn> Engine<Arn>
where
    Arn: Arena,
{
    #[must_use]
    pub(crate) fn from_tree(root: Combinator) -> Self {
        let mut arena = Arn::presized(10_000);
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

    pub(crate) fn unwind(&mut self, current: NodeKey) {
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

    pub(crate) fn reduce(&mut self) -> ReductionState {
        let Some(operator_key) = self.spine.pop() else {
            return ReductionState::Halted;
        };

        let Some(redex_root) = self.spine.pop() else {
            self.spine.push(operator_key);
            return ReductionState::Halted;
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
                        ReductionState::Reduced
                    } else {
                        ReductionState::Halted
                    }
                }
            },
            Some(x) => unreachable!("Tried reducing something that isn't a comb: {x:?}"),
            None => unreachable!(),
        };

        if matches!(result, ReductionState::Halted) {
            self.spine.push(redex_root);
            self.spine.push(operator_key);
        }

        result
    }

    fn reduce_three_args(&mut self, operator: NodeComb, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Halted;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            return ReductionState::Halted;
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
            ReductionState::Reduced
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            ReductionState::Halted
        }
    }

    fn reduce_two_args(&mut self, operator: NodeComb, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Halted;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(_y) = self.arena.get_arg(parent)
        {
            let result = match operator {
                NodeComb::K => Node::Indirection(x),
                _ => unreachable!("Tried reducing two args on a comb that doesn't take two"),
            };
            self.arena.replace(parent, result);
            ReductionState::Reduced
        } else {
            self.spine.push(parent);
            ReductionState::Halted
        }
    }
}

impl<Arn> fmt::Display for Engine<Arn>
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
