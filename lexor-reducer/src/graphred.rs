use core::{fmt::Debug, hash::Hash};

use lexor_parser::ski_parser::chumsky_parse as parse;
use rootcause::{Report, bail};
use slotmap::SlotMap;

use crate::{
    core::{
        engine::{Engine, ReductionState},
        node::{Node, NodeKey},
    },
    engineview::EngineView,
};

pub type RealArena = SlotMap<NodeKey, Node>;

/// The kind of reductions the [`ReductionMachine`] can perform.
///
/// Note that this trait is sealed.
pub trait ReductionStrat:
    Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + super::seal::Sealed
{
    fn perform(engine: &mut Engine<RealArena>, callback: &mut Option<impl FnMut(EngineView)>);

    /// Reduces the given `input` based on the strategy used to invoke this
    /// function and returns the result as a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use lexor_reducer::{NF, ReductionStrat};
    ///
    /// let result = NF::reduce("SKSKSSKSKSKSKSKSKSK").unwrap();
    ///
    /// assert_eq!(result, "K");
    /// ```
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn reduce(input: &str) -> Result<String, Report> {
        let Ok(root) = parse(input) else {
            bail!("Failed to parse input: {input}");
        };

        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut None::<fn(EngineView)>);
        Ok(format!("{}", EngineView::from_engine(&engine)))
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
    ///              }).unwrap();
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
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn reduce_with(input: &str, callback: impl FnMut(EngineView)) -> Result<String, Report> {
        let Ok(root) = parse(input) else {
            bail!("Failed to parse input: {input}");
        };

        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut Some(callback));
        Ok(format!("{}", EngineView::from_engine(&engine)))
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
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn compute(input: &str) -> Result<(), Report> {
        let Ok(root) = parse(input) else {
            bail!("Failed to parse input: {input}");
        };
        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut None::<fn(EngineView)>);
        Ok(())
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
    ///
    /// # Errors
    ///
    /// If the provided `input` cannot be parsed as a SKI expression.
    fn compute_with(input: &str, callback: impl FnMut(EngineView)) -> Result<(), Report> {
        let Ok(root) = parse(input) else {
            bail!("Failed to parse input: {input}");
        };

        let mut engine = Engine::from_tree(root);
        Self::perform(&mut engine, &mut Some(callback));
        Ok(())
    }
}

/// [Normal form] reduction strategy. Always reduces the current
/// leftmost-outermost redex to [`WeakHeadNormalForm`] until there are no more
/// saturated redexes in the whole term.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalForm;

impl Default for NormalForm {
    fn default() -> Self {
        unreachable!("NormalForm default")
    }
}

/// A type alias for [`NormalForm`].
pub type NF = NormalForm;

impl ReductionStrat for NormalForm {
    fn perform(engine: &mut Engine<RealArena>, callback: &mut Option<impl FnMut(EngineView)>) {
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
pub struct WeakHeadNormalForm;

impl Default for WeakHeadNormalForm {
    fn default() -> Self {
        unreachable!("WeakHeadNormalForm default")
    }
}

/// A type alias for [`WeakHeadNormalForm`].
pub type WHNF = WeakHeadNormalForm;

impl ReductionStrat for WeakHeadNormalForm {
    fn perform(engine: &mut Engine<RealArena>, callback: &mut Option<impl FnMut(EngineView)>) {
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
                f(EngineView::from_engine(engine));
            }
        }
    }
}
