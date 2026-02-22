use slotmap::SlotMap;

use crate::{
    core::arena::Arena,
    core::engine::{Engine, ReductionState},
    core::node::{Node, NodeKey},
    parse,
    parser::CombRec,
};

/// The kind of reductions the reducer can perform.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReductionMode {
    /// Always reduces the current leftmost-outermost redex to WHNF until there
    /// are no more saturated redexes in the whole term.
    #[default]
    NormalForm,
    /// Reduce the terms until the head of the term isn't saturated.
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
            mode: ReductionMode::NormalForm,
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

        format!("{}", self.engine)
    }

    pub fn compute(&mut self) {
        todo!()
    }

    pub(crate) fn reduce_with<F>(&mut self, callback: F) -> String
    where
        F: FnMut(&Engine<Arn>),
    {
        self.compute_with(callback);
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

    pub(crate) fn compute_with<F>(&mut self, mut callback: F)
    where
        F: FnMut(&Engine<Arn>),
    {
        match self.mode {
            ReductionMode::NormalForm => self.reduce_to_nf(&mut callback),
            ReductionMode::WeakHeadNormalForm => self.reduce_to_whnf(&mut callback),
        }
    }

    fn reduce_to_whnf<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&Engine<Arn>),
    {
        let mut prev_reduction = ReductionState::Reduced;

        while prev_reduction == ReductionState::Reduced {
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

            prev_reduction = self.engine.reduce();

            if prev_reduction == ReductionState::Reduced {
                callback(&self.engine);
            }
        }
    }

    fn reduce_to_nf<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&Engine<Arn>),
    {
        let mut work_stack = Vec::with_capacity(20);
        work_stack.push(self.engine.subtree_root);

        while let Some(next_root) = work_stack.pop() {
            self.engine.subtree_root = next_root;
            self.engine.spine.clear();

            self.reduce_to_whnf(callback);

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
///
/// # Reduction strategies
///
/// Currently there are two supported strategies:
/// - Weak Head Normal Form
/// - Normal form
///
/// More strategies might be added in the future.
///
/// # Combinators
///
/// Currently, these are the supported combinators and their definitions:
/// - **S** &nbsp;&nbsp;&nbsp; = &nbsp;&nbsp;&nbsp;`x -> y -> z -> x z(y z)`
/// - **K** &nbsp;&nbsp;&nbsp; = &nbsp;&nbsp;&nbsp;`x -> y -> x`
/// - **I** &nbsp;&nbsp;&nbsp; = &nbsp;&nbsp;&nbsp;`x -> x`
/// - **B** &nbsp;&nbsp;&nbsp; = &nbsp;&nbsp;&nbsp;`x -> y -> z -> x(y z)`
/// - **C** &nbsp;&nbsp;&nbsp; = &nbsp;&nbsp;&nbsp;`x -> y -> z -> x z y`
///
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
        self.reduce_with(|_| {})
    }

    /// Reduces the term loaded into the [`ReductionMachine`] and returns the
    /// result as a [`String`]. Executes `callback` after each reduction step.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use std::vec;
    /// use lexor_reducer::ReductionMachine;
    ///
    /// let mut logs: Vec<String> = vec![];
    /// let mut count: u32 = 0;
    ///
    /// let result =
    ///         ReductionMachine::from_string("S(SKSKSSK)(SSKSKSK)I")
    ///             .reduce_with(|view| {
    ///                 count = count.saturating_add(1);
    ///                 logs.push(format!("Step #{count}: {view}\n"));
    ///             });
    ///
    /// assert_eq!(result, "SKI");
    /// assert_eq!(logs.len(), 10);
    /// assert_eq!(logs[0], "Step #1: SKSKSSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[1], "Step #2: KK(SK)SSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[2], "Step #3: KSSKI(SSKSKSKI)\n");
    /// assert_eq!(logs[3], "Step #4: SKI(SSKSKSKI)\n");
    /// assert_eq!(logs[4], "Step #5: K(SSKSKSKI)(I(SSKSKSKI))\n");
    /// assert_eq!(logs[5], "Step #6: SSKSKSKI\n");
    /// assert_eq!(logs[6], "Step #7: SS(KS)KSKI\n");
    /// assert_eq!(logs[7], "Step #8: SK(KSK)SKI\n");
    /// assert_eq!(logs[8], "Step #9: KS(KSKS)KI\n");
    /// assert_eq!(logs[9], "Step #10: SKI\n");
    /// ```
    pub fn reduce_with<F>(&mut self, callback: F) -> String
    where
        F: FnMut(&Engine<ActualArena>),
    {
        self.inner.reduce_with(callback)
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
        self.compute_with(|_| {});
    }

    /// Computes the (weak head) normal form of the term loaded into the
    /// [`ReductionMachine`] but doesn't return anything. Executes `callback`
    /// after each reduction step. Mainly used for benchmarking.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::ReductionMachine;
    ///
    /// let mut machine = ReductionMachine::from_string("KSI");
    /// let mut count: u32 = 0;
    ///
    /// machine.compute_with(|_| { count = count.saturating_add(1); });
    ///
    /// assert_eq!(count, 1);
    /// assert_eq!(format!("{}", machine), "S");
    /// ```
    pub fn compute_with<F>(&mut self, callback: F)
    where
        F: FnMut(&Engine<ActualArena>),
    {
        self.inner.compute_with(callback);
    }
}

/// Returns the current reduction state.
///
/// # Example
///
/// ```
/// use lexor_reducer::ReductionMachine;
///
/// let machine = ReductionMachine::from_string("SKI");
///
/// assert_eq!(format!("{machine}"), "SKI");
/// ```
impl std::fmt::Display for ReductionMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
