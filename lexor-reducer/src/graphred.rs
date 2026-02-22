use core::{fmt::Debug, hash::Hash};

use slotmap::SlotMap;

use crate::{
    core::{
        engine::{Engine, ReductionState},
        node::{Node, NodeKey},
    },
    parse,
};

type RealArena = SlotMap<NodeKey, Node>;

/// The kind of reductions the [`ReductionMachine`] can perform.
///
/// Note that this trait is sealed.
pub trait ReductionStrat:
    Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + super::seal::Sealed
{
    fn perform<F: FnMut(&Engine<RealArena>)>(
        engine: &mut Engine<RealArena>,
        callback: &mut Option<F>,
    );

    /// Reduces the given `input` based on the strategy used to invoke this
    /// function and returns the result as a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let result = NF::reduce("SKSKSSKSKSKSKSKSKSK");
    ///
    /// assert_eq!(result, "K");
    /// ```
    fn reduce(input: &str) -> String {
        let root = parse(input).expect("Invalid input");
        let mut engine = Engine::from_tree(root);
        Self::perform::<fn(&Engine<RealArena>)>(&mut engine, &mut None);
        format!("{engine}")
    }

    /// Reduces the given `input` based on the strategy ued to invoke this
    /// function and returns the result as a [`String`]. Executes `callback`
    /// after each reduction step.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use std::vec;
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let mut logs: Vec<String> = vec![];
    /// let mut count: u32 = 0;
    ///
    /// let result = NF::reduce_with("S(SKSKSSK)(SSKSKSK)I", |view| {
    ///                  count = count.saturating_add(1);
    ///                  logs.push(format!("Step #{count}: {view}\n"));
    ///              });
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
    fn reduce_with<F: FnMut(&Engine<RealArena>)>(input: &str, callback: F) -> String {
        let root = parse(input).expect("Invalid input");
        let mut engine = Engine::from_tree(root);
        Self::perform::<F>(&mut engine, &mut Some(callback));
        format!("{engine}")
    }

    /// Computes the desired form of the given `input`.
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// // Internally, this reduces to "S"
    /// NF::compute("KSI");
    /// ```
    fn compute(input: &str) {
        let root = parse(input).expect("Invalid input");
        let mut engine = Engine::from_tree(root);
        Self::perform::<fn(&Engine<RealArena>)>(&mut engine, &mut None);
    }

    /// Computes the desired form of the given `input`. Executes `callback`
    /// after each reduction step.
    ///
    /// The type `Engine` implements the `Display` trait by returning
    /// the current reduction state. Thus you can view it as an "internal view".
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let mut count: u32 = 0;
    ///
    /// NF::compute_with("KSI", |_| { count = count.saturating_add(1); });
    ///
    /// assert_eq!(count, 1);
    /// ```
    fn compute_with<F: FnMut(&Engine<RealArena>)>(input: &str, callback: F) {
        let root = parse(input).expect("Invalid input");
        let mut engine = Engine::from_tree(root);
        Self::perform::<F>(&mut engine, &mut Some(callback));
    }
}

/// [Normal form] reduction strategy. Always reduces the current
/// leftmost-outermost redex to [`WeakHeadNormalForm`] until there are no more
/// saturated redexes in the whole term.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NormalForm {}

impl Default for NormalForm {
    fn default() -> Self {
        unreachable!("NormalForm default")
    }
}

/// A type alias for [`NormalForm`].
pub type NF = NormalForm;

impl ReductionStrat for NormalForm {
    fn perform<F>(engine: &mut Engine<RealArena>, callback: &mut Option<F>)
    where
        F: FnMut(&Engine<RealArena>),
    {
        let mut work_stack = Vec::with_capacity(20);
        work_stack.push(engine.subtree_root);

        while let Some(next_root) = work_stack.pop() {
            engine.subtree_root = next_root;
            engine.spine.clear();

            WHNF::perform(engine, callback);

            let _head = engine.spine.pop();
            for &app_node in &engine.spine {
                if let Some(Node::App(_lhs, rhs)) = engine.arena.get(app_node) {
                    work_stack.push(*rhs);
                }
            }
        }
    }
}

/// [Weak head normal form] reduction. Reduces until the head of the current
/// term isn't saturated (doesn't have enough arguments to reduce).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WeakHeadNormalForm {}

impl Default for WeakHeadNormalForm {
    fn default() -> Self {
        unreachable!("WeakHeadNormalForm default")
    }
}

/// A type alias for [`WeakHeadNormalForm`].
pub type WHNF = WeakHeadNormalForm;

impl ReductionStrat for WeakHeadNormalForm {
    fn perform<F>(engine: &mut Engine<RealArena>, callback: &mut Option<F>)
    where
        F: FnMut(&Engine<RealArena>),
    {
        let mut prev_reduction = ReductionState::Reduced;

        while prev_reduction == ReductionState::Reduced {
            let current = if let Some(&end) = engine.spine.last() {
                match engine.arena.get(end) {
                    Some(Node::App(lhs, _)) => *lhs,
                    Some(_) => end,
                    None => unreachable!(),
                }
            } else {
                engine.subtree_root
            };

            engine.unwind(current);

            prev_reduction = engine.reduce();

            if let Some(f) = callback
                && prev_reduction == ReductionState::Reduced
            {
                f(&engine);
            }
        }
    }
}
