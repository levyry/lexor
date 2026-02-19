#![allow(unused)]

use core::fmt;
use std::vec;

use slotmap::SlotMap;

use crate::{
    arena::Arena,
    engine::Engine,
    node::{Node, NodeComb, NodeKey},
    parse,
    parser::CombRec,
};

/// The kind of reduction the reducer can do.
///
/// - Weak Head Normal Form: Reduce the terms until the head isn't saturated.
/// - Normal Form: Always reduces the current leftmost-outermost redex to WHNF.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionMode {
    #[default]
    NormalForm,
    WeakHeadNormalForm,
}

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
        let mut current_redex_state = ReductionState::Reducable;

        while current_redex_state == ReductionState::Reducable {
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

/// A graph reduction machine capable of reducing SKI combinators.
pub struct ReductionMachine {
    inner: GenericGraphReductionMachine<ActualArena>,
}

impl ReductionMachine {
    /// Constructs an instance of [`ReductionMachine`] from the recursive, tree
    /// data structure provided by [`parse`].
    ///
    /// # Example
    ///
    /// ```
    /// let input = "SKI";
    /// let parsed = lexor_reducer::parse(input).unwrap();
    /// let engine = lexor_reducer::ReductionMachine::from_tree(parsed);
    /// ```
    #[must_use]
    pub fn from_tree(root: CombRec) -> Self {
        Self {
            inner: GenericGraphReductionMachine::from_tree(root),
        }
    }

    /// Construct an instance of [`ReductionMachine`] from a string of SKI
    /// terms.
    ///
    /// # Example
    ///
    /// ```
    /// let engine = lexor_reducer::ReductionMachine::from_string("SKI");
    /// ```
    #[must_use]
    pub fn from_string(term: &str) -> Self {
        Self {
            inner: GenericGraphReductionMachine::from_string(term),
        }
    }

    /// Sets the [`ReductionMode`] of the machine.
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{ReductionMachine, ReductionMode};
    ///
    /// let nf_result =
    ///     ReductionMachine::from_string("S(SS)(KK)(II)")
    ///     .set_mode(ReductionMode::NormalForm)
    ///     .reduce();
    ///
    /// let whnf_result =
    ///     ReductionMachine::from_string("S(SS)(KK)(II)")
    ///     .set_mode(ReductionMode::WeakHeadNormalForm)
    ///     .reduce();
    ///
    /// assert_eq!(nf_result, "SKK");
    /// assert_eq!(whnf_result, "S(KK(II))(II(KK(II)))");
    /// ```
    #[must_use]
    pub fn set_mode(self, new_mode: ReductionMode) -> Self {
        Self {
            inner: self.inner.set_mode(new_mode),
        }
    }

    /// Reduces the term loaded into the [`ReductionMachine`] and returns the
    /// result as a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::ReductionMachine;
    ///
    /// let result = ReductionMachine::from_string("SKSKSSKSKSKSKSKSKSK")
    ///         .reduce();
    ///
    /// assert_eq!(result, "K");
    /// ```
    pub fn reduce(&mut self) -> String {
        self.inner.reduce()
    }

    /// Computes the (weak head) normal form of the term loaded into the
    /// [`ReductionMachine`] but doesn't return anything. Mainly used for
    /// benchmarking.
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::ReductionMachine;
    ///
    /// let mut machine = ReductionMachine::from_string("KSI");
    ///
    /// machine.compute();
    ///
    /// // You can still get the result as a String from the Display impl.
    /// assert_eq!(format!("{}", machine), "S");
    /// ```
    pub fn compute(&mut self) {
        self.inner.compute();
    }
}

impl std::fmt::Display for ReductionMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
