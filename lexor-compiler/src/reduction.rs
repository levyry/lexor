use core::fmt;
use std::vec;

use crate::{
    arena::Arena,
    node::{Node, NodeKey},
    ski_parser::CombRec,
};

#[derive(Debug, Clone, Copy, Default)]
enum ReductionState {
    #[default]
    Reducible,
    Whnf,
}

#[derive(Debug, Clone, Copy, Default)]
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

    const fn set_mode(&mut self, new_mode: ReductionMode) {
        self.mode = new_mode;
    }

    fn reduce_head_to_whnf(&mut self) -> u64 {
        let mut steps: u64 = 0;

        while matches!(self.step(), ReductionState::Reducible) {
            steps = steps.saturating_add(1);
        }

        steps
    }

    fn start(&mut self) -> u64 {
        let mut steps: u64 = self.reduce_head_to_whnf();

        if matches!(self.mode, ReductionMode::NormalForm) {
            let _head = self.spine.pop();

            while let Some(redex_root) = self.spine.pop()
                && let Some(Node::App(_lhs, rhs)) = self.arena.get(redex_root)
            {
                let old_spine = self.spine.clone();
                self.spine = vec![];
                self.subtree_root = *rhs;
                let extra_steps = self.start();
                self.spine = old_spine;
                steps = steps.saturating_add(extra_steps);
            }
        }

        steps
    }

    fn step(&mut self) -> ReductionState {
        // Get current node to unwind from
        let current = if let Some(&parent) = self.spine.last() {
            match self.arena.get(parent) {
                Some(Node::App(lhs, _)) => *lhs,
                Some(_) => parent,
                None => unreachable!(),
            }
        } else {
            self.subtree_root
        };

        self.unwind(current);

        self.reduce()
    }

    fn unwind(&mut self, mut current: NodeKey) {
        loop {
            // TODO: Rewrite this to not `continue` so much
            if let Some(Node::Indirection(target)) = self.arena.get(current) {
                let target = *target;

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
                continue;
            }

            // Fill spine
            match self.arena.get(current) {
                Some(Node::App(lhs, _)) => {
                    self.spine.push(current);
                    current = *lhs;
                }
                Some(_) => {
                    self.spine.push(current);
                    break;
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
            Some(Node::I) => self.reduce_i(redex_root),
            Some(Node::K) => self.reduce_k(redex_root),
            Some(Node::S) => self.reduce_s(redex_root),
            // TODO: Finish B and C combinators
            _ => ReductionState::Whnf,
        };

        if matches!(result, ReductionState::Whnf) {
            self.spine.push(operator_key);
        }

        result
    }

    fn reduce_i(&mut self, redex_root: NodeKey) -> ReductionState {
        self.arena
            .get_arg(redex_root)
            .map_or(ReductionState::Whnf, |x| {
                self.arena.replace(redex_root, Node::Indirection(x));

                ReductionState::Reducible
            })
    }

    fn reduce_k(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            self.spine.push(redex_root);
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(_y) = self.arena.get_arg(parent)
        {
            self.arena.replace(parent, Node::Indirection(x));
            ReductionState::Reducible
        } else {
            self.spine.push(parent);
            self.spine.push(redex_root);
            ReductionState::Whnf
        }
    }

    fn reduce_s(&mut self, redex_root: NodeKey) -> ReductionState {
        let Some(parent) = self.spine.pop() else {
            self.spine.push(redex_root);
            return ReductionState::Whnf;
        };

        let Some(grandparent) = self.spine.pop() else {
            self.spine.push(parent);
            self.spine.push(redex_root);
            return ReductionState::Whnf;
        };

        if let Some(x) = self.arena.get_arg(redex_root)
            && let Some(y) = self.arena.get_arg(parent)
            && let Some(z) = self.arena.get_arg(grandparent)
        {
            let xz = self.arena.insert(Node::App(x, z));
            let yz = self.arena.insert(Node::App(y, z));

            self.arena.replace(grandparent, Node::App(xz, yz));
            ReductionState::Reducible
        } else {
            self.spine.push(grandparent);
            self.spine.push(parent);
            self.spine.push(redex_root);
            ReductionState::Whnf
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
                Some(Node::S) => write!(f, "S"),
                Some(Node::K) => write!(f, "K"),
                Some(Node::I) => write!(f, "I"),
                Some(Node::B) => write!(f, "B"),
                Some(Node::C) => write!(f, "C"),
                _ => todo!(),
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
        let parsed = ski_parser::parse(input).unwrap();
        let mut engine: GraphReductionEngine<SlotMap<NodeKey, Node>> =
            GraphReductionEngine::from_tree(parsed);

        engine.set_mode(ReductionMode::NormalForm);
        // engine.set_mode(ReductionMode::WeakHeadNormalForm);

        println!("Initial: {engine}");

        let steps = engine.start();

        println!("Final ({steps} steps): {engine}");

        assert_eq!(format!("{engine}"), "S(K(S(SS)K))(S(K(K(S(SS)K)))(S(SS)K))")
    }
}
