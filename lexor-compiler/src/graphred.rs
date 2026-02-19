use core::fmt;
use std::vec;

use crate::{
    arena::Arena,
    node::{Node, NodeComb, NodeKey},
    ski_parser::CombRec,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ReductionState {
    #[default]
    Reducable,
    Whnf,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ReductionMode {
    #[default]
    NormalForm,
    WeakHeadNormalForm,
}

#[derive(Debug, Clone)]
struct GraphReductionEngine<Arn: Arena> {
    root: NodeKey,
    subtree_root: NodeKey,
    spine: Vec<NodeKey>,
    arena: Arn,
    state: ReductionState,
    mode: ReductionMode,
}

impl<Arn> GraphReductionEngine<Arn>
where
    Arn: Arena + Default,
{
    #[must_use]
    fn from_tree(root: CombRec) -> Self {
        let mut arena = Arn::default();
        let root = arena.flatten(root);

        Self {
            root,
            subtree_root: root,
            spine: vec![],
            arena,
            state: ReductionState::default(),
            mode: ReductionMode::default(),
        }
    }

    const fn set_mode(mut self, new_mode: ReductionMode) -> Self {
        self.mode = new_mode;
        self
    }

    fn start(&mut self) {
        let mut current_redex_state = ReductionState::Reducable;

        while current_redex_state == ReductionState::Reducable {
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

        if matches!(self.mode, ReductionMode::NormalForm) {
            let _head = self.spine.pop();

            while let Some(&redex_root) = self.spine.last()
                && let Some(Node::App(_lhs, rhs)) = self.arena.get(redex_root)
            {
                // Performance: at this point, self.spine nearly always contains
                // either one or two `NodeKey`s, so cloning it shouldn't be that
                // much of a performance hit. juggling with multiple vectors
                // might actually be slower tbh
                let old_spine = self.spine.clone();
                self.spine = vec![];
                self.subtree_root = *rhs;

                self.start();
                // steps = steps.saturating_add(extra_steps);

                self.spine = old_spine;
                self.spine.pop();
            }
        }
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
            return ReductionState::Whnf;
        };

        let Some(redex_root) = self.spine.pop() else {
            self.spine.push(operator_key);
            return ReductionState::Whnf;
        };

        let operator_node = self.arena.get(operator_key);

        let result = match operator_node {
            Some(Node::Comb(operator)) => match operator {
                NodeComb::S => self.reduce_s(redex_root),
                NodeComb::K => self.reduce_k(redex_root),
                NodeComb::I => self.reduce_i(redex_root),
                NodeComb::B => self.reduce_b(redex_root),
                NodeComb::C => self.reduce_c(redex_root),
                NodeComb::Sn(_) => todo!(),
                NodeComb::Bn(_) => todo!(),
                NodeComb::Cn(_) => todo!(),
            },
            Some(x) => unreachable!("Tried reducing something that isn't a comb: {x:?}"),
            None => unreachable!(),
        };

        if matches!(result, ReductionState::Whnf) {
            self.spine.push(redex_root);
            self.spine.push(operator_key);
        }

        result
    }

    fn reduce_i(&mut self, redex_root: NodeKey) -> ReductionState {
        if let Some(x) = self.arena.get_arg(redex_root) {
            self.arena.replace(redex_root, Node::Indirection(x));
            ReductionState::Reducable
        } else {
            ReductionState::Whnf
        }
    }

    fn reduce_k(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(_y) = self.arena.get_arg(parent)
        {
            self.arena.replace(parent, Node::Indirection(x));
            ReductionState::Reducable
        } else {
            self.spine.push(parent);
            ReductionState::Whnf
        }
    }

    fn reduce_s(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Whnf;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(y) = self.arena.get_arg(parent)
            && let Some(z) = self.arena.get_arg(grandparent)
        {
            let xz = self.arena.insert(Node::App(x, z));
            let yz = self.arena.insert(Node::App(y, z));

            self.arena.replace(grandparent, Node::App(xz, yz));
            ReductionState::Reducable
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            ReductionState::Whnf
        }
    }

    fn reduce_b(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Whnf;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(y) = self.arena.get_arg(parent)
            && let Some(z) = self.arena.get_arg(grandparent)
        {
            let yz = self.arena.insert(Node::App(y, z));

            self.arena.replace(grandparent, Node::App(x, yz));
            ReductionState::Reducable
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            ReductionState::Whnf
        }
    }

    fn reduce_c(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            return ReductionState::Whnf;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(y) = self.arena.get_arg(parent)
            && let Some(z) = self.arena.get_arg(grandparent)
        {
            let xz = self.arena.insert(Node::App(x, z));

            self.arena.replace(grandparent, Node::App(xz, y));
            ReductionState::Reducable
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            ReductionState::Whnf
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

#[cfg(test)]
mod tests {
    use slotmap::SlotMap;

    use crate::ski_parser;

    use super::*;

    #[test]
    fn check() {
        let input = "S(SSS)S(SS)K";
        // let input = "S(SS)(KK)(II)";
        let parsed = ski_parser::parse(input).unwrap();
        let mut engine: GraphReductionEngine<SlotMap<NodeKey, Node>> =
            GraphReductionEngine::from_tree(parsed).set_mode(ReductionMode::NormalForm);

        engine.start();

        println!("Final: {engine}");

        assert_eq!(format!("{engine}"), "S(K(S(SS)K))(S(K(K(S(SS)K)))(S(SS)K))");
    }
}
