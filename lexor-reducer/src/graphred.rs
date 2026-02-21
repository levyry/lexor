#![allow(unused)]

use core::fmt;
use std::vec;

use slotmap::SlotMap;

use crate::{
    arena::Arena,
    engine::{Engine, ReductionState},
    node::{Node, NodeComb, NodeKey},
    parse,
    parser::CombRec,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionMode {
    #[default]
    NormalForm,
    WeakHeadNormalForm,
}

#[derive(Debug, Clone)]
pub struct GenericGraphReductionMachine<Arn>
where
    Arn: Arena,
{
    mode: ReductionMode,
    engine: Engine<Arn>,
}

impl<Arn> GenericGraphReductionMachine<Arn>
where
    Arn: Arena,
{
    #[must_use]
    pub fn from_tree(root: CombRec) -> Self {
        let engine = Engine::from_tree(root);

        Self {
            mode: ReductionMode::default(),
            engine,
        }
    }

    #[must_use]
    ///
    /// # Panics
    ///
    pub fn from_string(term: &str) -> Self {
        // FIXME: return a result with proper error types later
        let tree = parse(term).expect("Failed to parse input");
        Self::from_tree(tree)
    }

    #[must_use]
    pub const fn set_mode(mut self, new_mode: ReductionMode) -> Self {
        self.mode = new_mode;
        self
    }

    pub fn reduce(&mut self) -> String {
        self.compute();
        // TODO: If the given ski term is sufficiently large, then transforming
        // it into a string might take longer than the reduction itself. A more
        // performant way of providing the result should be implemented. A few
        // possible options include:
        //
        // 1. Using streams instead of allocating one big String
        // 2. Finding a good "BigString" crate, or something similar
        // 3. Only returning the final Arena, and letting the user do whatever
        // 4. Providing a "view" of the result, or a cursor
        // 5. Trying to optimize the current Display impl
        format!("{}", self.engine)
    }

    pub fn compute(&mut self) {
        match self.mode {
            ReductionMode::NormalForm => self.reduce_to_nf(),
            ReductionMode::WeakHeadNormalForm => self.reduce_to_whnf(),
        }
    }

    fn reduce_to_whnf(&mut self) {
        let mut current_redex_state = ReductionState::Reducible;

        while current_redex_state == ReductionState::Reducible {
            let current = if let Some(&end) = self.engine.spine.last() {
                match self.engine.arena.get(end) {
                    Some(Node::App(lhs, _)) => *lhs,
                    Some(_) => end,
                    None => unreachable!(),
                }
            } else {
                self.engine.subtree_root
            };

            self.engine.unwind(current);

            current_redex_state = self.engine.reduce();
        }
    }

    fn reduce_to_nf(&mut self) {
        let mut work_stack = Vec::with_capacity(20);
        work_stack.push(self.engine.subtree_root);

        while let Some(next_root) = work_stack.pop() {
            self.engine.subtree_root = next_root;
            self.engine.spine.clear();

            self.reduce_to_whnf();

            let _head = self.engine.spine.pop();
            for &app_node in &self.engine.spine {
                if let Some(Node::App(_lhs, rhs)) = self.engine.arena.get(app_node) {
                    work_stack.push(*rhs);
                }
            }
        }
    }
}

impl<Arn> std::fmt::Display for GenericGraphReductionMachine<Arn>
where
    Arn: Arena,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.engine)
    }
}

// Public facing API
type ActualArena = SlotMap<NodeKey, Node>;
pub struct ReductionMachine {
    inner: GenericGraphReductionMachine<ActualArena>,
}

impl ReductionMachine {
    #[must_use]
    pub fn from_tree(root: CombRec) -> Self {
        Self {
            inner: GenericGraphReductionMachine::from_tree(root),
        }
    }

    #[must_use]
    pub fn from_string(term: &str) -> Self {
        Self {
            inner: GenericGraphReductionMachine::from_string(term),
        }
    }

    #[must_use]
    pub fn set_mode(self, new_mode: ReductionMode) -> Self {
        Self {
            inner: self.inner.set_mode(new_mode),
        }
    }

    pub fn reduce(&mut self) -> String {
        self.inner.reduce()
    }

    pub fn compute(&mut self) {
        self.inner.compute();
    }
}

impl std::fmt::Display for ReductionMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
