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
        self.reduce_to_whnf();

        let _head = self.engine.spine.pop();

        while let Some(&redex_root) = self.engine.spine.last()
            && let Some(Node::App(_lhs, rhs)) = self.engine.arena.get(redex_root)
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
            let old_spine = std::mem::take(&mut self.engine.spine);
            self.engine.subtree_root = *rhs;

            self.reduce_to_nf();

            self.engine.spine = old_spine;
            self.engine.spine.pop();
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
